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
  (document "standard_view_definitions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 18) (end 4 23))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwView,KwDef,OpenAngle,Ident,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwView,KwDef,OpenAngle,Ident,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwView,KwDef,OpenAngle,Ident,CloseAngle,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwView,KwDef,OpenAngle,Ident,CloseAngle,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwView,KwDef,OpenAngle,Ident,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwView,KwDef,OpenAngle,Ident,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwView,KwDef,OpenAngle,Ident,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwView,KwDef,OpenAngle,Ident,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'StandardViewDefinitions'
    (documentation)
    (import_decl public 'SysML::*')
    (view_def 'GeneralView'
      (documentation))
    (view_def 'InterconnectionView'
      (documentation))
    (view_def 'ActionFlowView' :> 'InterconnectionView'
      (documentation))
    (view_def 'StateTransitionView' :> 'InterconnectionView'
      (documentation))
    (view_def 'SequenceView'
      (documentation))
    (view_def 'GeometryView'
      (documentation))
    (view_def 'GridView'
      (documentation))
    (view_def 'BrowserView'
      (documentation))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2a9826190845fc2431f58cccead96f0a9101f6641920f75c4be583f6a8541773") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions"))) (kind "package") (name "StandardViewDefinitions") (declared-name "StandardViewDefinitions") (range (start (line 0) (character 0)) (end (line 0) (character 6114))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 4)) (end (line 4) (character 27))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions"))) (authored (membership (kind Import) (visibility "public") (import (reference "SysML::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 18)) (end (line 4) (character 23))))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (kind "view def") (name "ActionFlowView") (declared-name "ActionFlowView") (range (start (line 32) (character 4)) (end (line 32) (character 946))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "InterconnectionView") (range (start (line 32) (character 46)) (end (line 32) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView::_documentation"))) (kind "documentation") (name "") (range (start (line 32) (character 4)) (end (line 32) (character 946))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::BrowserView"))) (kind "view def") (name "BrowserView") (declared-name "BrowserView") (range (start (line 113) (character 4)) (end (line 113) (character 494))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::BrowserView::_documentation"))) (kind "documentation") (name "") (range (start (line 113) (character 4)) (end (line 113) (character 494))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions::BrowserView"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::GeneralView"))) (kind "view def") (name "GeneralView") (declared-name "GeneralView") (range (start (line 6) (character 4)) (end (line 6) (character 1342))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::GeneralView::_documentation"))) (kind "documentation") (name "") (range (start (line 6) (character 4)) (end (line 6) (character 1342))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions::GeneralView"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::GeometryView"))) (kind "view def") (name "GeometryView") (declared-name "GeometryView") (range (start (line 80) (character 4)) (end (line 80) (character 1016))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::GeometryView::_documentation"))) (kind "documentation") (name "") (range (start (line 80) (character 4)) (end (line 80) (character 1016))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions::GeometryView"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::GridView"))) (kind "view def") (name "GridView") (declared-name "GridView") (range (start (line 102) (character 4)) (end (line 102) (character 459))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::GridView::_documentation"))) (kind "documentation") (name "") (range (start (line 102) (character 4)) (end (line 102) (character 459))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions::GridView"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (kind "view def") (name "InterconnectionView") (declared-name "InterconnectionView") (range (start (line 23) (character 4)) (end (line 23) (character 359))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView::_documentation"))) (kind "documentation") (name "") (range (start (line 23) (character 4)) (end (line 23) (character 359))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::SequenceView"))) (kind "view def") (name "SequenceView") (declared-name "SequenceView") (range (start (line 63) (character 4)) (end (line 63) (character 847))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::SequenceView::_documentation"))) (kind "documentation") (name "") (range (start (line 63) (character 4)) (end (line 63) (character 847))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions::SequenceView"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (kind "view def") (name "StateTransitionView") (declared-name "StateTransitionView") (range (start (line 52) (character 4)) (end (line 52) (character 446))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "InterconnectionView") (range (start (line 52) (character 51)) (end (line 52) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView::_documentation"))) (kind "documentation") (name "") (range (start (line 52) (character 4)) (end (line 52) (character 446))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView"))))
    (element (id (node (document "d0") (qualified-name "StandardViewDefinitions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 6114))) (parent (node (document "d0") (qualified-name "StandardViewDefinitions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "StandardViewDefinitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SysML::*") (range (start (line 4) (character 18)) (end (line 4) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (kind specialization) (ordinal 0)) (authored-target "InterconnectionView") (range (start (line 32) (character 46)) (end (line 32) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (kind specialization) (ordinal 0)) (authored-target "InterconnectionView") (range (start (line 52) (character 51)) (end (line 52) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (target (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (target (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
