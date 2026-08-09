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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "StandardViewDefinitions"))) (name "StandardViewDefinitions") (declared-name "StandardViewDefinitions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "StandardViewDefinitions::*"))) (name "*") (declared-name "*"))
        (element (kind "view def") (id (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (name "ActionFlowView") (declared-name "ActionFlowView")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView")))))
          )
        )
        (element (kind "view def") (id (node (document "d0") (qualified-name "StandardViewDefinitions::BrowserView"))) (name "BrowserView") (declared-name "BrowserView")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StandardViewDefinitions::BrowserView::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StandardViewDefinitions::BrowserView")))))
          )
        )
        (element (kind "view def") (id (node (document "d0") (qualified-name "StandardViewDefinitions::GeneralView"))) (name "GeneralView") (declared-name "GeneralView")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StandardViewDefinitions::GeneralView::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StandardViewDefinitions::GeneralView")))))
          )
        )
        (element (kind "view def") (id (node (document "d0") (qualified-name "StandardViewDefinitions::GeometryView"))) (name "GeometryView") (declared-name "GeometryView")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StandardViewDefinitions::GeometryView::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StandardViewDefinitions::GeometryView")))))
          )
        )
        (element (kind "view def") (id (node (document "d0") (qualified-name "StandardViewDefinitions::GridView"))) (name "GridView") (declared-name "GridView")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StandardViewDefinitions::GridView::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StandardViewDefinitions::GridView")))))
          )
        )
        (element (kind "view def") (id (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (name "InterconnectionView") (declared-name "InterconnectionView")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
          )
        )
        (element (kind "view def") (id (node (document "d0") (qualified-name "StandardViewDefinitions::SequenceView"))) (name "SequenceView") (declared-name "SequenceView")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StandardViewDefinitions::SequenceView::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StandardViewDefinitions::SequenceView")))))
          )
        )
        (element (kind "view def") (id (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (name "StateTransitionView") (declared-name "StateTransitionView")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "StandardViewDefinitions::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView::_documentation"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::BrowserView::_documentation"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions::BrowserView"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::GeneralView::_documentation"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions::GeneralView"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::GeometryView::_documentation"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions::GeometryView"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::GridView::_documentation"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions::GridView"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView::_documentation"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::SequenceView::_documentation"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions::SequenceView"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView::_documentation"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::_documentation"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::ActionFlowView"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (to (node (document "d0") (qualified-name "StandardViewDefinitions::InterconnectionView"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
