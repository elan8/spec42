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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 0) (end 2 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 3 0) (end 3 31))
      )
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
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 5 0) (end 5 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 0) (end 6 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 9 4) (end 9 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 10 4) (end 10 24))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 15 4) (end 16 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 16 4) (end 16 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 18 4) (end 18 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:78b35de4d04c636bb29de5b8d361ee7022e2ce9eb901ed20f63f349bfb04f71d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "AbstractVehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "Container"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "Container::InnerPart"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "Container::InnerWeight"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "EngineChoices"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_abstract_defs.md") (qualified-name "Weight"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
