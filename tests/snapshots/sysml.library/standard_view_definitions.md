# META
~~~ini
description=Standard Library: Systems Library/StandardViewDefinitions
type=file
~~~
# SOURCE
~~~sysml
standard library package StandardViewDefinitions {
    doc /*
         * This package defines the standard view definitions for the SysML language.
         */
    public import SysML::*;

    view def <gv> GeneralView {
        doc /*
             * View definition to present any members of exposed model element(s).
             * This is the most general view, enabling presentation of any model element.
             * The typical rendering in graphical notation is as a graph of nodes and edges.
             * Specializations of GeneralView can be specified through appropriate selection of filters, e.g.:
             * - package view, filtering on Package, Package containment, package Import
             * - definition and usage view, filtering on Definition, Usage, Specialization, FeatureTyping (covering defined by)
             * - requirement view, filtering on RequirementDefinition, RequirementUsage, Specialization, FeatureTyping, 
             *   SatisfyRequirementUsage, AllocationDefinition, AllocationUsage,
             * - view and viewpoint view, filtering on ViewDefinition, ViewUsage, ViewpointDefinition, ViewpointUsage, 
             *   RenderingDefinition, RenderingUsage, ConcernDefinition, ConcernUsage, StakeholderMembership, ...
             * - language extension view, filtering on Metaclass, MetadataFeature, MetadataAccessExpression, ...
             * Note: filters are specified by referencing concepts from the KerML.kerml and SysML.sysml standard library packages.
             */
    }

    view def <iv> InterconnectionView {
        doc /*
             * View definition to present exposed features as nodes, nested features as
             * nested nodes, and connections between features
             * as edges between (nested) nodes. Nested nodes may present boundary features
             * (e.g., ports, parameters).
             */
    }

    view def <afv> ActionFlowView specializes InterconnectionView {
        doc /*
             * View definition to present connections between actions.
             * Valid nodes and edges in an ActionFlowView are:
             * - Actions with nested actions
             * - Parameters with direction
             * - Flow connection usages (e.g., kinds of transfers from output to input)
             * - Binding connections between parameters (e.g., delegate a parameter from
             *   one level of nesting to another)
             * - Proxy connection points
             * - Swim lanes
             * - Conditional succession
             * - Control nodes (fork, join, decision, merge)
             * - Control structures, e.g., if-then-else, until-while-loop, for-loop
             * - Send and accept actions
             * - Change and time triggers
             * - Compartments on actions and parameters
             */
    }

    view def <stv> StateTransitionView specializes InterconnectionView {
        doc /*
             * View definition to present states and their transitions.
             * Valid nodes and edges in a StateTransitionView are:
             * - States with nested states
             * - Entry, do, and exit actions
             * - Transition usages with triggers, guards, and actions
             * - Compartments on states
             */
    }

    view def <sv> SequenceView {
        doc /*
             * View definition to present time ordering of event occurrences on lifelines
             * of exposed features.
             * Valid nodes and edges in a SequenceView are:
             * - Features such as parts with their lifelines
             * - Event occurrences on the lifelines
             * - Messages sent from one part to another with and without a type of flow
             * - Succession between event occurrences
             * - Nested sequence view (e.g., a reference to a view)
             * - Compartments
             * The typical rendering in graphical notation depicts the exposed features
             * horizontally along the top, with vertical lifelines. The time axis is
             * vertical, with time increasing from top to bottom.
             */
    }

    view def <gev> GeometryView {
        doc /*
             * View definition to present a visualization of exposed spatial items in two
             * or three dimensions
             * Valid nodes and edges in a GeometryView are:
             * - Spatial item, including shape
             * - Coordinate frame
             * - Feature related to spatial item, such as a quantity (e.g. temperature)
             *   of which values are to be rendered on a color scale
             * The typical rendering in graphical notation would include a number of
             * visualization parameters, such as:
             * - 2D or 3D view
             * - viewing direction
             * - zoom level
             * - light sources
             * - object projection mode, e.g., isometric, perspective, orthographic
             * - object rendering mode, e.g., shaded, wireframe, hidden line
             * - object pan (placement) and rotate (orientation) settings
             * - color maps
             */
    }

    view def <grv> GridView {
        doc /*
             * View definition to present exposed model elements and their relationships,
             * arranged in a rectangular grid.
             * GridView is the generalization of the following more specialized views:
             * - Tabular view
             * - Data value tabular view
             * - Relationship matrix view, e.g. presenting allocation or dependency relationships
             */
    }

    view def <bv> BrowserView {
        doc /*
             * View definition to present the hierarchical membership structure of model
             * elements starting from one or more exposed root elements.
             * The typical rendering in graphical notation is as an indented list of rows,
             * consisting of dynamically collapsible-expandable nodes that represent
             * branches and leaves of the tree, as in graphical user interface widgets.
             */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/standard_view_definitions.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 4 18) (end 4 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 18) (end 4 26))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:311c30428a7fffa832eba1c3be29d9fd4c11faa3fffa06175c93299b7411638e") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n         * This package defines the standard view definitions for the SysML language.\n         "))))
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (path (named (kind library-package) (name "StandardViewDefinitions")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "SysML") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (kind view-def) (membership (kind owning) (visibility default)) (facts (short-name "afv")) (documentation (doc (text "\n             * View definition to present connections between actions.\n             * Valid nodes and edges in an ActionFlowView are:\n             * - Actions with nested actions\n             * - Parameters with direction\n             * - Flow connection usages (e.g., kinds of transfers from output to input)\n             * - Binding connections between parameters (e.g., delegate a parameter from\n             *   one level of nesting to another)\n             * - Proxy connection points\n             * - Swim lanes\n             * - Conditional succession\n             * - Control nodes (fork, join, decision, merge)\n             * - Control structures, e.g., if-then-else, until-while-loop, for-loop\n             * - Send and accept actions\n             * - Change and time triggers\n             * - Compartments on actions and parameters\n             "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::BrowserView"))) (kind view-def) (membership (kind owning) (visibility default)) (facts (short-name "bv")) (documentation (doc (text "\n             * View definition to present the hierarchical membership structure of model\n             * elements starting from one or more exposed root elements.\n             * The typical rendering in graphical notation is as an indented list of rows,\n             * consisting of dynamically collapsible-expandable nodes that represent\n             * branches and leaves of the tree, as in graphical user interface widgets.\n             "))))
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (kind view-def) (membership (kind owning) (visibility default)) (facts (short-name "gv")) (documentation (doc (text "\n             * View definition to present any members of exposed model element(s).\n             * This is the most general view, enabling presentation of any model element.\n             * The typical rendering in graphical notation is as a graph of nodes and edges.\n             * Specializations of GeneralView can be specified through appropriate selection of filters, e.g.:\n             * - package view, filtering on Package, Package containment, package Import\n             * - definition and usage view, filtering on Definition, Usage, Specialization, FeatureTyping (covering defined by)\n             * - requirement view, filtering on RequirementDefinition, RequirementUsage, Specialization, FeatureTyping, \n             *   SatisfyRequirementUsage, AllocationDefinition, AllocationUsage,\n             * - view and viewpoint view, filtering on ViewDefinition, ViewUsage, ViewpointDefinition, ViewpointUsage, \n             *   RenderingDefinition, RenderingUsage, ConcernDefinition, ConcernUsage, StakeholderMembership, ...\n             * - language extension view, filtering on Metaclass, MetadataFeature, MetadataAccessExpression, ...\n             * Note: filters are specified by referencing concepts from the KerML.kerml and SysML.sysml standard library packages.\n             "))))
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeometryView"))) (kind view-def) (membership (kind owning) (visibility default)) (facts (short-name "gev")) (documentation (doc (text "\n             * View definition to present a visualization of exposed spatial items in two\n             * or three dimensions\n             * Valid nodes and edges in a GeometryView are:\n             * - Spatial item, including shape\n             * - Coordinate frame\n             * - Feature related to spatial item, such as a quantity (e.g. temperature)\n             *   of which values are to be rendered on a color scale\n             * The typical rendering in graphical notation would include a number of\n             * visualization parameters, such as:\n             * - 2D or 3D view\n             * - viewing direction\n             * - zoom level\n             * - light sources\n             * - object projection mode, e.g., isometric, perspective, orthographic\n             * - object rendering mode, e.g., shaded, wireframe, hidden line\n             * - object pan (placement) and rotate (orientation) settings\n             * - color maps\n             "))))
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView"))) (kind view-def) (membership (kind owning) (visibility default)) (facts (short-name "grv")) (documentation (doc (text "\n             * View definition to present exposed model elements and their relationships,\n             * arranged in a rectangular grid.\n             * GridView is the generalization of the following more specialized views:\n             * - Tabular view\n             * - Data value tabular view\n             * - Relationship matrix view, e.g. presenting allocation or dependency relationships\n             "))))
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (kind view-def) (membership (kind owning) (visibility default)) (facts (short-name "iv")) (documentation (doc (text "\n             * View definition to present exposed features as nodes, nested features as\n             * nested nodes, and connections between features\n             * as edges between (nested) nodes. Nested nodes may present boundary features\n             * (e.g., ports, parameters).\n             "))))
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::SequenceView"))) (kind view-def) (membership (kind owning) (visibility default)) (facts (short-name "sv")) (documentation (doc (text "\n             * View definition to present time ordering of event occurrences on lifelines\n             * of exposed features.\n             * Valid nodes and edges in a SequenceView are:\n             * - Features such as parts with their lifelines\n             * - Event occurrences on the lifelines\n             * - Messages sent from one part to another with and without a type of flow\n             * - Succession between event occurrences\n             * - Nested sequence view (e.g., a reference to a view)\n             * - Compartments\n             * The typical rendering in graphical notation depicts the exposed features\n             * horizontally along the top, with vertical lifelines. The time axis is\n             * vertical, with time increasing from top to bottom.\n             "))))
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (kind view-def) (membership (kind owning) (visibility default)) (facts (short-name "stv")) (documentation (doc (text "\n             * View definition to present states and their transitions.\n             * Valid nodes and edges in a StateTransitionView are:\n             * - States with nested states\n             * - Entry, do, and exit actions\n             * - Transition usages with triggers, guards, and actions\n             * - Compartments on states\n             "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "InterconnectionView")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/standard_view_definitions.md") (path (named (kind library-package) (name "StandardViewDefinitions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SysML")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (kind specialization) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (kind specialization) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (target (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (target (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")))
      (supertype (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))
      (subtype (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))
      (supertype (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/standard_view_definitions.md") (range (start 4 18) (end 4 26)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/standard_view_definitions.md") (path (named (kind library-package) (name "StandardViewDefinitions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "SysML")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/standard_view_definitions.md") (range (start 32 46) (end 32 65)) (probe (position 32 46))
    (reference (id (source (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (kind specialization) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/standard_view_definitions.md") (range (start 52 51) (end 52 70)) (probe (position 52 51))
    (reference (id (source (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (kind specialization) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
)
~~~
