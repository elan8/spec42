# META
~~~ini
description=A composite action is drawn as a container holding its own action flow
type=generate
libraries=standard
plugin=native:diagram
viewKind=action-flow-view
viewDocument=model.sysml
viewQualifiedName=Shop::fulfilment
~~~
# SOURCE
## model.sysml
~~~sysml
package Shop {
    private import StandardViewDefinitions::*;

    action def Fulfil {
        action order {
            action pick;
            action pack;
            first pick then pack;
        }
        action ship;
        first order then ship;
    }

    view fulfilment : ActionFlowView {
        expose Fulfil;
    }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:61ab6b3edc306cfaa2079793d2c415de72fa5c52194b9df7e27de0e0555ff7be") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "order")) (succession (reference "ship")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "pick")) (succession (reference "pack")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pack"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pick"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::ship"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::fulfilment"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ActionFlowView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind view) (name "fulfilment")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Fulfil")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "order")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "ship")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::ship")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "pick")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pick")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "pack")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pack")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::fulfilment"))) (kind featureTyping) (ordinal 0))
      (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind view) (name "fulfilment")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Fulfil")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::ship"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pick"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pack"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::fulfilment"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::fulfilment"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind view) (name "fulfilment")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind view) (name "fulfilment")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pack"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pack"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pack"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pick"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pick"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pick"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::ship"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::ship"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::ship"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::fulfilment"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind view) (name "fulfilment")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::fulfilment"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pack")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pick")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::ship")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::fulfilment")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind view) (name "fulfilment")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::fulfilment")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 14) (end 10 19)) (probe (position 10 14))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "order")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 25) (end 10 29)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "ship")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::ship")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 18) (end 7 22)) (probe (position 7 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "pick")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pick")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 28) (end 7 32)) (probe (position 7 28))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind action-def) (name "Fulfil")) (named (kind action) (name "order")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "pack")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil::order::pack")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 22) (end 13 36)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::fulfilment"))) (kind featureTyping) (ordinal 0) (authored-target "ActionFlowView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 15) (end 14 21)) (probe (position 14 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Shop")) (named (kind view) (name "fulfilment")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Fulfil")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Shop::Fulfil")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 6,
  "modelDigest": "blake3:0d8712dfd443c0b8cbe2494000b33487d48498afef9dcf94776e0d54371d04be",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        15,
        3,
        21
      ]
    },
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
        19,
        5,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        19,
        6,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        12,
        7,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        18,
        7,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        28,
        7,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        15,
        9,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        8,
        10,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        14,
        10,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        25,
        10,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        9,
        13,
        19
      ]
    }
  ],
  "references": [
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Shop::Fulfil"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Shop::Fulfil::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Shop::Fulfil::order"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Shop::Fulfil::order::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Shop::Fulfil::order::pack"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Shop::Fulfil::order::pick"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Shop::Fulfil::ship"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Shop::fulfilment"
    },
    {
      "kind": "source-anchor",
      "source": 8,
      "ownerQualifiedName": "Shop::Fulfil",
      "metaclass": "SuccessionAsUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 4,
      "ownerQualifiedName": "Shop::Fulfil::order",
      "metaclass": "SuccessionAsUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 0,
      "relationshipKind": "containment",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "succession",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "succession",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "containment",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "containment",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "containment",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "succession",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "succession",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "succession",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "succession",
      "ordinal": 5
    }
  ],
  "selectedView": {
    "reference": 7,
    "kind": "action-flow-view",
    "name": "fulfilment",
    "source": 11
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "kind": "action-flow-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 0,
        "metaclass": "ActionDefinition",
        "notationRole": "definition",
        "name": "Fulfil",
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 0,
        "compartments": [
          {
            "kind": "actions",
            "provenance": "direct",
            "members": [
              2,
              3
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 8,
        "metaclass": "SuccessionAsUsage",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 8,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 6,
        "metaclass": "ActionUsage",
        "notationRole": "usage",
        "name": "ship",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 7,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 2,
        "metaclass": "ActionUsage",
        "notationRole": "usage",
        "name": "order",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 1,
        "compartments": [
          {
            "kind": "actions",
            "provenance": "direct",
            "members": [
              5,
              6
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 9,
        "metaclass": "SuccessionAsUsage",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 3,
        "source": 4,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 4,
        "metaclass": "ActionUsage",
        "notationRole": "usage",
        "name": "pack",
        "typing": {
          "status": "absent"
        },
        "owner": 3,
        "source": 3,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "ActionUsage",
        "notationRole": "usage",
        "name": "pick",
        "typing": {
          "status": "absent"
        },
        "owner": 3,
        "source": 2,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      }
    ],
    "relationships": [
      {
        "reference": 13,
        "source": 1,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 3
        },
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 14,
        "source": 1,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 2
        },
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 19,
        "source": 4,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 6
        },
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 20,
        "source": 4,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 5
        },
        "provenance": "authored",
        "navigation": 6
      }
    ],
    "edges": [
      {
        "reference": 10,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 18,
        "source": 3,
        "target": 2,
        "origin": 1,
        "kind": "succession",
        "provenance": "implied",
        "navigation": 9
      },
      {
        "reference": 11,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 12,
        "source": 0,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 15,
        "source": 3,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 4
      },
      {
        "reference": 21,
        "source": 6,
        "target": 5,
        "origin": 4,
        "kind": "succession",
        "provenance": "implied",
        "navigation": 5
      },
      {
        "reference": 16,
        "source": 3,
        "target": 5,
        "origin": 5,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 17,
        "source": 3,
        "target": 6,
        "origin": 6,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 2
      }
    ],
    "metadata": {
      "actions": [
        0,
        2,
        3,
        5,
        6
      ],
      "controlNodes": []
    },
    "scene": {
      "kind": "action-flow"
    }
  }
}

~~~
