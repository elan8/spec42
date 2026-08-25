# META
~~~ini
description=SysML ActionDefinition action retains ActionUsage values from effective usage
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.3:deriveActionDefinitionAction
libraries=none
~~~
# SOURCE
~~~sysml
package Actions {
    action def Base {
        action inherited;
        action retained;
        part notAction;
    }
    action def Procedure specializes Base {
        action replacement :>> inherited;
        action step;
        state mode;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.3:deriveActionDefinitionAction") (source "Actions::Procedure") (target "Actions::Base::retained") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.3:deriveActionDefinitionAction") (source "Actions::Procedure") (target "Actions::Procedure::mode") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.3:deriveActionDefinitionAction") (source "Actions::Procedure") (target "Actions::Procedure::replacement") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.3:deriveActionDefinitionAction") (source "Actions::Procedure") (target "Actions::Procedure::step") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_action_definition_action.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 4 8) (end 4 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4fbf8ce0c5a117f3bd8817a0e97878e77a61fc34134fad5a50b7951ac20b6833") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::inherited"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::notAction"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::retained"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::mode"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::replacement"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "inherited")))))
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::step"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::replacement"))) (kind redefinition) (ordinal 0))
      (authored-target "inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::inherited")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure"))) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::replacement"))) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::inherited"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::replacement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::inherited"))) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::notAction"))) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::retained"))) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::mode"))) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::replacement"))) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::step"))) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base")))
      (subtype (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::inherited")))
      (featured-by (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base")))
      (subtype (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::replacement")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::notAction")))
      (featured-by (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::retained")))
      (featured-by (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure")))
      (supertype (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::mode")))
      (featured-by (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::replacement")))
      (featured-by (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure")))
      (supertype (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::inherited")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::step")))
      (featured-by (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_action_definition_action.md") (range (start 6 37) (end 6 41)) (probe (position 6 37))
    (reference (id (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_action_definition_action.md") (range (start 7 31) (end 7 40)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::replacement"))) (kind redefinition) (ordinal 0) (authored-target "inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Base::inherited")))))
    )
  )
)
~~~
