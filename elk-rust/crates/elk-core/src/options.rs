use crate::geometry::Size;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LayoutDirection {
    #[default]
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HierarchyHandling {
    #[default]
    IncludeChildren,
    IgnoreChildren,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PortConstraint {
    #[default]
    Free,
    FixedSide,
    FixedOrder,
    FixedPosition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EdgeRouting {
    Straight,
    #[default]
    Orthogonal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NodeAlignment {
    Start,
    Center,
    End,
    #[default]
    Balanced,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LayerConstraint {
    #[default]
    None,
    First,
    Last,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ContentAlignment {
    Start,
    #[default]
    Center,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NodeLabelPlacement {
    InsideTopLeft,
    InsideTopCenter,
    InsideTopRight,
    #[default]
    OutsideTopCenter,
    OutsideBottomCenter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PortLabelPlacement {
    Inside,
    #[default]
    Outside,
    NextToPortIfPossible,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EdgeLabelPlacement {
    Head,
    Tail,
    #[default]
    Center,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ViewProfile {
    #[default]
    Default,
    GeneralView,
    InterconnectionView,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    #[must_use]
    pub const fn uniform(value: f32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    #[must_use]
    pub const fn horizontal_vertical(horizontal: f32, vertical: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    #[must_use]
    pub fn size(self) -> Size {
        Size::new(self.left + self.right, self.top + self.bottom)
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::uniform(24.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spacing {
    pub node_spacing: f32,
    pub layer_spacing: f32,
    pub edge_spacing: f32,
    pub segment_spacing: f32,
    pub label_spacing: f32,
    pub port_label_spacing: f32,
    pub component_spacing: f32,
    pub label_clearance: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            node_spacing: 40.0,
            layer_spacing: 80.0,
            edge_spacing: 24.0,
            segment_spacing: 20.0,
            label_spacing: 10.0,
            port_label_spacing: 8.0,
            component_spacing: 120.0,
            label_clearance: 12.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LayeredOptions {
    pub direction: LayoutDirection,
    pub spacing: Spacing,
    pub padding: Padding,
    pub edge_routing: EdgeRouting,
    pub hierarchy_handling: HierarchyHandling,
    pub respect_port_order: bool,
    pub node_alignment: NodeAlignment,
    pub prioritize_straight_edges: bool,
    pub compactness: f32,
    pub component_packing: bool,
    pub preferred_connector_lanes: usize,
}

impl Default for LayeredOptions {
    fn default() -> Self {
        Self {
            direction: LayoutDirection::default(),
            spacing: Spacing::default(),
            padding: Padding::default(),
            edge_routing: EdgeRouting::default(),
            hierarchy_handling: HierarchyHandling::default(),
            respect_port_order: true,
            node_alignment: NodeAlignment::default(),
            prioritize_straight_edges: true,
            compactness: 0.7,
            component_packing: false,
            preferred_connector_lanes: 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ElementLayoutOptions {
    pub direction: Option<LayoutDirection>,
    pub edge_routing: Option<EdgeRouting>,
    pub hierarchy_handling: Option<HierarchyHandling>,
    pub port_constraint: Option<PortConstraint>,
    pub respect_port_order: Option<bool>,
    pub node_alignment: Option<NodeAlignment>,
    pub layer_constraint: Option<LayerConstraint>,
    pub content_alignment: Option<ContentAlignment>,
    pub node_label_placement: Option<NodeLabelPlacement>,
    pub port_label_placement: Option<PortLabelPlacement>,
    pub edge_label_placement: Option<EdgeLabelPlacement>,
    pub padding: Option<Padding>,
    pub spacing: Option<Spacing>,
    pub model_order: Option<usize>,
    /// Edges with the same bundle key and shared source or target connect at the same point on nodes.
    pub edge_bundle_key: Option<u32>,
}

impl ElementLayoutOptions {
    #[must_use]
    pub fn inherit_from(&self, parent: &Self) -> Self {
        Self {
            direction: self.direction.or(parent.direction),
            edge_routing: self.edge_routing.or(parent.edge_routing),
            hierarchy_handling: self.hierarchy_handling.or(parent.hierarchy_handling),
            port_constraint: self.port_constraint.or(parent.port_constraint),
            respect_port_order: self.respect_port_order.or(parent.respect_port_order),
            node_alignment: self.node_alignment.or(parent.node_alignment),
            layer_constraint: self.layer_constraint.or(parent.layer_constraint),
            content_alignment: self.content_alignment.or(parent.content_alignment),
            node_label_placement: self.node_label_placement.or(parent.node_label_placement),
            port_label_placement: self.port_label_placement.or(parent.port_label_placement),
            edge_label_placement: self.edge_label_placement.or(parent.edge_label_placement),
            padding: self.padding.or(parent.padding),
            spacing: self.spacing.or(parent.spacing),
            model_order: self.model_order.or(parent.model_order),
            edge_bundle_key: self.edge_bundle_key.or(parent.edge_bundle_key),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LayoutOptions {
    pub layered: LayeredOptions,
    pub element_defaults: ElementLayoutOptions,
    pub view_profile: ViewProfile,
}

impl LayoutOptions {
    #[must_use]
    pub fn resolve(&self, overrides: &ElementLayoutOptions) -> ElementLayoutOptions {
        overrides.inherit_from(&self.element_defaults)
    }

    #[must_use]
    pub fn with_view_profile(mut self, profile: ViewProfile) -> Self {
        self.view_profile = profile;
        self.apply_view_profile_defaults();
        self
    }

    pub fn apply_view_profile_defaults(&mut self) {
        match self.view_profile {
            ViewProfile::Default => {}
            ViewProfile::GeneralView => {
                self.layered.direction = LayoutDirection::TopToBottom;
                self.layered.edge_routing = EdgeRouting::Orthogonal;
                self.layered.spacing.node_spacing = 56.0;
                self.layered.spacing.layer_spacing = 96.0;
                self.layered.spacing.component_spacing = 160.0;
                self.layered.spacing.label_spacing = 12.0;
                self.layered.spacing.label_clearance = 16.0;
                self.layered.compactness = 0.58;
                self.layered.component_packing = true;
                self.layered.preferred_connector_lanes = 2;
                self.element_defaults.content_alignment = Some(ContentAlignment::Start);
                self.element_defaults.node_label_placement =
                    Some(NodeLabelPlacement::OutsideTopCenter);
            }
            ViewProfile::InterconnectionView => {
                self.layered.direction = LayoutDirection::TopToBottom;
                self.layered.edge_routing = EdgeRouting::Orthogonal;
                self.layered.spacing.node_spacing = 44.0;
                self.layered.spacing.layer_spacing = 88.0;
                self.layered.spacing.edge_spacing = 28.0;
                self.layered.spacing.segment_spacing = 24.0;
                self.layered.spacing.port_label_spacing = 10.0;
                self.layered.spacing.component_spacing = 132.0;
                self.layered.spacing.label_clearance = 18.0;
                self.layered.compactness = 0.72;
                self.layered.component_packing = true;
                self.layered.preferred_connector_lanes = 5;
                self.element_defaults.port_constraint = Some(PortConstraint::FixedOrder);
                self.element_defaults.port_label_placement =
                    Some(PortLabelPlacement::NextToPortIfPossible);
                self.element_defaults.edge_label_placement = Some(EdgeLabelPlacement::Center);
            }
        }
    }
}
