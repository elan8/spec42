# META
~~~ini
description=An untyped nested action is an Actions::Action through its implied subactions subsetting, so its own start and done are drawn inside it and the outer flow keeps the outer start and done; SysML 2.0 8.2.3.17, KerML 8.3.3.3.3
type=generate
libraries=standard
plugin=native:diagram
viewKind=action-flow-view
viewDocument=model.sysml
viewQualifiedName=Nested::driveFlow
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
## model.sysml
~~~sysml
package Nested {
    private import StandardViewDefinitions::*;
    action def Burn;
    action def Transmit;
    action def Drive {
        action propel {
            first start;
            then action burn : Burn;
            then done;
        }
        action transmit : Transmit;
        first start then propel;
        first propel then transmit;
        first transmit then done;
    }
    view driveFlow : ActionFlowView { expose Drive; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3758d78bd656e102c15c2c0ec557dfd536491b0d8d97c0a39a1a7056e04453d3") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Burn"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")) (succession (reference "propel")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "propel")) (succession (reference "transmit")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "transmit")) (succession (reference "done")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "start")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "done")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel::burn"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Burn")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmit")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Transmit"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::driveFlow"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ActionFlowView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind view) (name "driveFlow")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Drive")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "propel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0))
      (authored-target "transmit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "propel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "transmit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel::burn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Burn")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Burn")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Transmit")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::driveFlow"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind view) (name "driveFlow")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Drive")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel::burn"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Burn"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel::burn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Transmit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::driveFlow"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::driveFlow"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind view) (name "driveFlow")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind view) (name "driveFlow")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Burn"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel::burn"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel::burn"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel::burn"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Transmit"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::driveFlow"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind view) (name "driveFlow")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::driveFlow"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Burn")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel::burn")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind then-continuation) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel::burn")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Burn")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Burn")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Burn")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Transmit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Transmit")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Transmit")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Transmit")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::driveFlow")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind view) (name "driveFlow")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::driveFlow")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 14) (end 11 19)) (probe (position 11 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 14) (end 12 20)) (probe (position 12 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "propel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 14) (end 13 22)) (probe (position 13 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0) (authored-target "transmit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 25) (end 11 31)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "propel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 26) (end 12 34)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "transmit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 28) (end 13 32)) (probe (position 13 28))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 18) (end 6 23)) (probe (position 6 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "start")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::start")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 17) (end 8 21)) (probe (position 8 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind action-def) (name "Drive")) (named (kind action) (name "propel")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::done")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 31) (end 7 35)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::propel::burn"))) (kind featureTyping) (ordinal 0) (authored-target "Burn")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Burn")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 26) (end 10 34)) (probe (position 10 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive::transmit"))) (kind featureTyping) (ordinal 0) (authored-target "Transmit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Transmit")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 21) (end 15 35)) (probe (position 15 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::driveFlow"))) (kind featureTyping) (ordinal 0) (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 45) (end 15 50)) (probe (position 15 45))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Nested")) (named (kind view) (name "driveFlow")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Drive")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Nested::Drive")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:43a023d81e869dd0facd00e63971cb4b51b88ca4fa7ab321c89d1eedae9ea34b",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/actions.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/occurrences.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        4,
        15,
        4,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        15,
        5,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        12,
        6,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        18,
        6,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        24,
        7,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        31,
        7,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        12,
        8,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        17,
        8,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        15,
        10,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        26,
        10,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        8,
        11,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        14,
        11,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        25,
        11,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        8,
        12,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        14,
        12,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        26,
        12,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        8,
        13,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        14,
        13,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        28,
        13,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        9,
        15,
        18
      ]
    },
    {
      "document": 1,
      "range": [
        42,
        9,
        42,
        14
      ]
    },
    {
      "document": 1,
      "range": [
        42,
        16,
        42,
        22
      ]
    },
    {
      "document": 1,
      "range": [
        42,
        27,
        42,
        36
      ]
    },
    {
      "document": 1,
      "range": [
        49,
        9,
        49,
        13
      ]
    },
    {
      "document": 1,
      "range": [
        49,
        15,
        49,
        21
      ]
    },
    {
      "document": 1,
      "range": [
        49,
        26,
        49,
        33
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Nested::Burn"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Nested::Drive"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Nested::Drive::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Nested::Drive::propel"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Nested::Drive::propel::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Nested::Drive::propel::burn"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Nested::Drive::transmit"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Nested::Transmit"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Nested::driveFlow"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::Action"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::Action::done"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::Action::start"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence::endShot"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence::startShot"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Nested::Drive",
      "source": 10,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Nested::Drive",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Nested::Drive",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Nested::Drive::propel",
      "source": 2,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Nested::Drive::propel",
      "source": 6,
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
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 16,
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
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "thenTarget",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "succession",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "succession",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "redefinition",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "redefinition",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "typing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "typing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "redefinition",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "redefinition",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "succession",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "succession",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "typing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "typing",
      "source": 11
    }
  ],
  "selectedView": {
    "reference": 8,
    "kind": "action-flow-view",
    "name": "driveFlow",
    "source": 19
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 1,
        "provenance": "authored",
        "reference": 19,
        "source": 0,
        "target": 1
      },
      {
        "kind": "succession",
        "navigation": 11,
        "origin": 1,
        "provenance": "implied",
        "reference": 50,
        "source": 12,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 2,
        "provenance": "authored",
        "reference": 20,
        "source": 0,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 14,
        "origin": 2,
        "provenance": "implied",
        "reference": 37,
        "source": 4,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 3,
        "provenance": "authored",
        "reference": 21,
        "source": 0,
        "target": 3
      },
      {
        "kind": "succession",
        "navigation": 17,
        "origin": 3,
        "provenance": "implied",
        "reference": 42,
        "source": 10,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 4,
        "provenance": "authored",
        "reference": 22,
        "source": 0,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 5,
        "provenance": "authored",
        "reference": 32,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 7,
        "provenance": "authored",
        "reference": 33,
        "source": 4,
        "target": 7
      },
      {
        "kind": "succession",
        "navigation": 4,
        "origin": 7,
        "provenance": "authored",
        "reference": 51,
        "source": 9,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 23,
        "origin": 8,
        "provenance": "implied",
        "reference": 34,
        "source": 4,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 20,
        "origin": 9,
        "provenance": "implied",
        "reference": 35,
        "source": 4,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 6,
        "provenance": "authored",
        "reference": 36,
        "source": 4,
        "target": 6
      },
      {
        "kind": "succession",
        "navigation": 6,
        "origin": 6,
        "provenance": "authored",
        "reference": 40,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 10,
        "provenance": "authored",
        "reference": 23,
        "source": 0,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 23,
        "origin": 11,
        "provenance": "implied",
        "reference": 24,
        "source": 0,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 20,
        "origin": 12,
        "provenance": "implied",
        "reference": 25,
        "source": 0,
        "target": 12
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
        7,
        8,
        9,
        10,
        11,
        12
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
              10
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              11,
              12
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionDefinition",
        "multiplicity": null,
        "name": "Drive",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:87ea6a7fe25c3da5019d3e08f10996ea72347e32e69cdfd5eaa8c534047ed03c",
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
        "presentationKey": "p:954f49baa17381d6b25772056a373b55210db7934558e237e9ad2c8212b1f9d3",
        "reference": 14,
        "source": 10,
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
        "presentationKey": "p:ab505040dd34ca8913bf8758ec36bfda505eee36585ff7f263b638d6fbd359e7",
        "reference": 15,
        "source": 13,
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
        "presentationKey": "p:efa85882d2dc690c2765d1e9b2e96fa5064b4c1055a5099804b66e175bca298a",
        "reference": 16,
        "source": 16,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              7
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              8,
              9
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "propel",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:99f276f5eb758fc1478080e836fbe14f4ea6a671209bc0aeb6e9f32f8bf8c956",
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 4,
        "presentationKey": "p:6e8f7c8972fd33e95e2c6a82813a311300114ecb0502b77a89916eba0388ec81",
        "reference": 17,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 4,
        "presentationKey": "p:3807b4f3c6766bb1b081aefcd60fd2caaac5cc00d14ed4e01af1a37eff34c850",
        "reference": 18,
        "source": 6,
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
        "name": "burn",
        "notationRole": "usage",
        "owner": 4,
        "presentationKey": "p:ecd91dd0ac6c738c04ffa4f3f70cea8ab167f309e391a0429c8388e2f4ad6bbd",
        "reference": 5,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Burn",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "done",
        "notationRole": "usage",
        "owner": 4,
        "presentationKey": "p:c6611ffbd166378137c097581130058dd41cf935d8d21fbc68482bb7b3b8bf8a",
        "reference": 10,
        "source": 23,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "start",
        "notationRole": "usage",
        "owner": 4,
        "presentationKey": "p:7258247df34b2f13a58c5a70e1f9bf0ec2d741458e789fd21d75aa4e1b14f2f8",
        "reference": 11,
        "source": 20,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "transmit",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:e7bbf7a984d4bc6349aa76cdcfc8b81af2d415c074a6ce374ef05760dfa6d881",
        "reference": 6,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Transmit",
              "reference": 7
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "done",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:a5cc5e33443da51fad18b07fae7fc7045cdf33f818f99a1a9f5acfdbc01eeff0",
        "reference": 10,
        "source": 23,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "start",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:887041b2f627cd74109445e8e7efad94c899ce42c79fedc7ba46697de645cd88",
        "reference": 11,
        "source": 20,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Action",
              "reference": 9
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "succession",
        "navigation": 11,
        "provenance": "authored",
        "reference": 26,
        "source": 1,
        "target": {
          "node": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 12,
        "provenance": "authored",
        "reference": 27,
        "source": 1,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 14,
        "provenance": "authored",
        "reference": 28,
        "source": 2,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 15,
        "provenance": "authored",
        "reference": 29,
        "source": 2,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 17,
        "provenance": "authored",
        "reference": 30,
        "source": 3,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 18,
        "provenance": "authored",
        "reference": 31,
        "source": 3,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 3,
        "provenance": "authored",
        "reference": 38,
        "source": 5,
        "target": {
          "node": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 41,
        "source": 7,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 24,
        "provenance": "authored",
        "reference": 46,
        "source": 8,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 25,
        "provenance": "authored",
        "reference": 44,
        "source": 8,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 21,
        "provenance": "authored",
        "reference": 52,
        "source": 9,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 22,
        "provenance": "authored",
        "reference": 48,
        "source": 9,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "thenTarget",
        "navigation": 7,
        "provenance": "authored",
        "reference": 39,
        "source": 6,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 43,
        "source": 10,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 24,
        "provenance": "authored",
        "reference": 47,
        "source": 11,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 25,
        "provenance": "authored",
        "reference": 45,
        "source": 11,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 21,
        "provenance": "authored",
        "reference": 53,
        "source": 12,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 22,
        "provenance": "authored",
        "reference": 49,
        "source": 12,
        "target": {
          "reference": 13,
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
