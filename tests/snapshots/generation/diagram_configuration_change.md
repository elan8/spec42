# META
~~~ini
description=Action flow keeps view identity distinct from an equally named exposed action
type=generate
libraries=standard
plugin=native:diagram
viewKind=action-flow-view
viewDocument=diagram_configuration_change.md
viewQualifiedName=ScenarioViews::configurationChange
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package Scenarios {
    action configurationChange {
        action authenticate;
        action reviewCompatibility;
        action commit;
        action proveRestart;
        first authenticate then reviewCompatibility;
        first reviewCompatibility then commit;
        first commit then proveRestart;
    }
}
package ScenarioViews {
    private import StandardViewDefinitions::*;
    view configurationChange : ActionFlowView { expose Scenarios::configurationChange; }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_configuration_change.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_configuration_change.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f14ce8a90114a53b09f00a743cf435523f2bdacff537683be825ab722543c4d0") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "ScenarioViews"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "ScenarioViews")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "ScenarioViews::configurationChange"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ActionFlowView")))))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "ScenarioViews")) (named (kind view) (name "configurationChange")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Scenarios::configurationChange")))))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "authenticate")) (succession (reference "reviewCompatibility")))))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "reviewCompatibility")) (succession (reference "commit")))))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "commit")) (succession (reference "proveRestart")))))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::authenticate"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::proveRestart"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "ScenarioViews")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "ScenarioViews::configurationChange"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "ScenarioViews")) (named (kind view) (name "configurationChange")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Scenarios::configurationChange")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange")))))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "authenticate")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::authenticate")))))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "reviewCompatibility")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility")))))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0))
      (authored-target "commit")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit")))))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "reviewCompatibility")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility")))))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "commit")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit")))))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1))
      (authored-target "proveRestart")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::proveRestart")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "ScenarioViews::configurationChange"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "ScenarioViews::configurationChange"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "ScenarioViews")) (named (kind view) (name "configurationChange")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "ScenarioViews")) (named (kind view) (name "configurationChange")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::authenticate"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::proveRestart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "ScenarioViews::configurationChange"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "ScenarioViews")) (named (kind view) (name "configurationChange")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "ScenarioViews::configurationChange"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::authenticate"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::authenticate"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::authenticate"))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit"))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::proveRestart"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::proveRestart"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::proveRestart"))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility"))) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "ScenarioViews::configurationChange")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "ScenarioViews")) (named (kind view) (name "configurationChange")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "ScenarioViews::configurationChange")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Without")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Without")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Without")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::authenticate")))
      (featured-by (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit")))
      (featured-by (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::proveRestart")))
      (featured-by (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility")))
      (featured-by (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_configuration_change.md") (range (start 12 19) (end 12 45)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "ScenarioViews")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_configuration_change.md") (range (start 13 31) (end 13 45)) (probe (position 13 31))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "ScenarioViews::configurationChange"))) (kind featureTyping) (ordinal 0) (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    )
  )
  (query (document "memory://snapshot/diagram_configuration_change.md") (range (start 13 55) (end 13 85)) (probe (position 13 55))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "ScenarioViews")) (named (kind view) (name "configurationChange")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Scenarios::configurationChange")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange")))))
    )
  )
  (query (document "memory://snapshot/diagram_configuration_change.md") (range (start 6 14) (end 6 26)) (probe (position 6 14))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "authenticate")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::authenticate")))))
    )
  )
  (query (document "memory://snapshot/diagram_configuration_change.md") (range (start 7 14) (end 7 33)) (probe (position 7 14))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "reviewCompatibility")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility")))))
    )
  )
  (query (document "memory://snapshot/diagram_configuration_change.md") (range (start 8 14) (end 8 20)) (probe (position 8 14))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0) (authored-target "commit")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit")))))
    )
  )
  (query (document "memory://snapshot/diagram_configuration_change.md") (range (start 6 32) (end 6 51)) (probe (position 6 32))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "reviewCompatibility")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::reviewCompatibility")))))
    )
  )
  (query (document "memory://snapshot/diagram_configuration_change.md") (range (start 7 39) (end 7 45)) (probe (position 7 39))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "commit")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::commit")))))
    )
  )
  (query (document "memory://snapshot/diagram_configuration_change.md") (range (start 8 26) (end 8 38)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/diagram_configuration_change.md") (path (named (kind package) (name "Scenarios")) (named (kind action) (name "configurationChange")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1) (authored-target "proveRestart")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_configuration_change.md") (qualified-name "Scenarios::configurationChange::proveRestart")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 10,
  "modelDigest": "blake3:b110bacc440fe16a2e7c53c694bfcd12081ed8bde923559ef0ca1cf39783f48d",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_configuration_change.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        1,
        11,
        1,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        2,
        15,
        2,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        15,
        3,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        15,
        4,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        15,
        5,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        8,
        6,
        52
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        14,
        6,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        32,
        6,
        51
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        8,
        7,
        46
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        14,
        7,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        39,
        7,
        45
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        8,
        8,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        14,
        8,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        26,
        8,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        9,
        13,
        28
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "ScenarioViews::configurationChange"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Scenarios::configurationChange"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Scenarios::configurationChange::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Scenarios::configurationChange::authenticate"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Scenarios::configurationChange::commit"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Scenarios::configurationChange::proveRestart"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Scenarios::configurationChange::reviewCompatibility"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Scenarios::configurationChange",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Scenarios::configurationChange",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Scenarios::configurationChange",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "succession",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "succession",
      "source": 6
    }
  ],
  "selectedView": {
    "reference": 0,
    "kind": "action-flow-view",
    "name": "configurationChange",
    "source": 14
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 1,
        "provenance": "authored",
        "reference": 10,
        "source": 0,
        "target": 1
      },
      {
        "kind": "succession",
        "navigation": 6,
        "origin": 1,
        "provenance": "implied",
        "reference": 23,
        "source": 4,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 2,
        "provenance": "authored",
        "reference": 11,
        "source": 0,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 9,
        "origin": 2,
        "provenance": "implied",
        "reference": 25,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 3,
        "provenance": "authored",
        "reference": 12,
        "source": 0,
        "target": 3
      },
      {
        "kind": "succession",
        "navigation": 12,
        "origin": 3,
        "provenance": "implied",
        "reference": 24,
        "source": 7,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 7,
        "provenance": "authored",
        "reference": 13,
        "source": 0,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 4,
        "provenance": "authored",
        "reference": 14,
        "source": 0,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 5,
        "provenance": "authored",
        "reference": 15,
        "source": 0,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 6,
        "provenance": "authored",
        "reference": 16,
        "source": 0,
        "target": 6
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "action-flow-view",
    "metadata": {
      "actions": [
        0,
        4,
        5,
        6,
        7
      ],
      "controlNodes": []
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              4,
              5,
              6,
              7
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "configurationChange",
        "notationRole": "usage",
        "owner": null,
        "reference": 1,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 7,
        "source": 5,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 8,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 9,
        "source": 11,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "authenticate",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "proveRestart",
        "notationRole": "usage",
        "owner": 0,
        "reference": 5,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "reviewCompatibility",
        "notationRole": "usage",
        "owner": 0,
        "reference": 6,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "commit",
        "notationRole": "usage",
        "owner": 0,
        "reference": 4,
        "source": 3,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "succession",
        "navigation": 6,
        "provenance": "authored",
        "reference": 17,
        "source": 1,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 7,
        "provenance": "authored",
        "reference": 18,
        "source": 1,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 9,
        "provenance": "authored",
        "reference": 19,
        "source": 2,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 10,
        "provenance": "authored",
        "reference": 20,
        "source": 2,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 12,
        "provenance": "authored",
        "reference": 21,
        "source": 3,
        "target": {
          "node": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 13,
        "provenance": "authored",
        "reference": 22,
        "source": 3,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "action-flow"
    }
  }
}

~~~
