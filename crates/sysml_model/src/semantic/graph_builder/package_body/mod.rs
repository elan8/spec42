use std::collections::HashMap;

use sysml_v2_parser::ast::{
    AliasDef, AllocationDef, AllocationUsage, AnalysisCaseDef, AnalysisCaseUsage, AttributeDef,
    CaseDef, CaseUsage, ClassifierDecl, ConcernUsage, ConnectionDef, ConnectionDefBody, Dependency,
    EnumDef, EnumeratedValue, EnumerationBody, EnumerationUsage, FeatureDecl, Import,
    IndividualDef, InterfaceDef, InterfaceDefBody, ItemDef, MetadataDef, OccurrenceDef,
    PackageBodyElement, PartDef, PartDefBody, PortDef, PortDefBody, RequirementDef, StateDef,
    StateDefBody, StateUsage, TextualRepresentation, UseCaseDef, UseCaseDefBody, UseCaseUsage,
    VerificationCaseDef, VerificationCaseUsage,
};
use sysml_v2_parser::{Node, RootNamespace};
use url::Url;

use super::requirement_body::{import_member_label, walk_requirement_def_body};
use crate::semantic::ast_util::{
    attach_short_name_attribute, definition_feature_properties, identification_name, span_to_range,
    text_range_to_json,
};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{ElementKind, NodeId, RelationshipKind};
use crate::semantic::relationships::{add_typing_edge_if_exists, try_wire_derivation_connection};

use super::action;
use super::analysis_case;
use super::attribute_body;
use super::calc_constraint_def;
use super::definition_body;
use super::expressions;
use super::kerml_library;
use super::modeled_kerml_name::extract_modeled_decl_name;
use super::package_packages;
use super::unit_metadata;
use super::verification;
use super::view_def;
use super::{
    add_node_and_recurse, attach_feature_properties, insert_def_specialization_attr,
    qualified_name_for_node, wire_def_specialization_edge,
};
use super::{interface_def, part_def, port_def, state, usage_builders, use_case};

mod materialize;
pub(crate) use materialize::*;

pub(super) fn build_from_package_body_element(
    node: &Node<PackageBodyElement>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    root: &RootNamespace,
    g: &mut SemanticGraph,
) {
    use sysml_v2_parser::ast::PackageBodyElement as PBE;
    match &node.value {
        PBE::Package(pkg_node) => {
            package_packages::build_nested_package(
                pkg_node,
                uri,
                container_prefix,
                parent_id,
                root,
                g,
                build_from_package_body_element,
            );
        }
        PBE::LibraryPackage(pkg_node) => {
            package_packages::build_nested_library_package(
                pkg_node,
                uri,
                container_prefix,
                parent_id,
                root,
                g,
                build_from_package_body_element,
            );
        }
        PBE::PartDef(pd_node) => materialize_part_def(g, uri, container_prefix, parent_id, pd_node),
        PBE::PartUsage(pu_node) => {
            usage_builders::materialize_part_usage(pu_node, uri, container_prefix, parent_id, g);
        }
        PBE::FeatureDecl(feature_node) => {
            materialize_feature_decl(g, uri, container_prefix, parent_id, feature_node)
        }
        PBE::ClassifierDecl(classifier_node) => {
            materialize_classifier_decl(g, uri, container_prefix, parent_id, classifier_node)
        }
        PBE::PortDef(pd_node) => materialize_port_def(g, uri, container_prefix, parent_id, pd_node),
        PBE::InterfaceDef(id_node) => {
            materialize_interface_def(g, uri, container_prefix, parent_id, id_node)
        }
        PBE::AttributeDef(ad_node) => {
            materialize_attribute_def(g, uri, container_prefix, parent_id, ad_node)
        }
        PBE::AttributeUsage(attribute) => {
            if let Some(parent_id) = parent_id {
                usage_builders::materialize_attribute_usage(
                    attribute,
                    uri,
                    container_prefix,
                    parent_id,
                    g,
                );
            }
        }
        PBE::PortUsage(port) => {
            if let Some(parent_id) = parent_id {
                port_def::materialize_port_usage(port, uri, container_prefix, parent_id, g);
            }
        }
        PBE::ActionDef(ad_node) => {
            let name = identification_name(&ad_node.identification);
            let qualified = action::materialize_action_def(
                g,
                uri,
                container_prefix,
                parent_id,
                ad_node,
                &name,
                ad_node.specializes.as_deref(),
            );
            wire_def_specialization_edge(
                g,
                uri,
                &qualified,
                container_prefix,
                ad_node.specializes.as_deref(),
            );
        }
        PBE::ActionUsage(au_node) => {
            action::materialize_top_level_action_usage(
                g,
                uri,
                container_prefix,
                parent_id,
                au_node,
            );
        }
        PBE::AliasDef(alias_node) => {
            materialize_alias_def(g, uri, container_prefix, parent_id, alias_node)
        }
        PBE::RequirementDef(rd_node) => {
            materialize_requirement_def(g, uri, container_prefix, parent_id, rd_node)
        }
        PBE::RequirementUsage(ru_node) => {
            usage_builders::materialize_requirement_usage(
                ru_node,
                uri,
                container_prefix,
                parent_id,
                g,
            );
        }
        PBE::Satisfy(satisfy_node) => materialize_satisfy(g, uri, container_prefix, satisfy_node),
        PBE::AllocationUsage(alloc_node) => {
            materialize_allocation_usage(g, uri, container_prefix, parent_id, alloc_node)
        }
        PBE::ConcernUsage(cu_node) => {
            materialize_concern_usage(g, uri, container_prefix, parent_id, cu_node)
        }
        PBE::UseCaseDef(ucd_node) => {
            materialize_use_case_def(g, uri, container_prefix, parent_id, ucd_node)
        }
        PBE::UseCaseUsage(ucu_node) => {
            materialize_use_case_usage(g, uri, container_prefix, parent_id, ucu_node)
        }
        PBE::ItemDef(item_node) => {
            materialize_item_def(g, uri, container_prefix, parent_id, item_node)
        }
        PBE::IndividualDef(ind_node) => {
            materialize_individual_def(g, uri, container_prefix, parent_id, ind_node)
        }
        PBE::MetadataDef(md_node) => {
            materialize_metadata_def(g, uri, container_prefix, parent_id, md_node)
        }
        PBE::MetadataUsage(mu_node) => {
            if let Some(parent_id) = parent_id {
                super::metadata_def::add_package_metadata_usage_node(
                    g,
                    uri,
                    container_prefix,
                    parent_id,
                    mu_node,
                    &mu_node.span,
                );
            }
        }
        PBE::EnumDef(enum_node) => {
            materialize_enum_def(g, uri, container_prefix, parent_id, enum_node)
        }
        PBE::OccurrenceDef(occ_node) => {
            materialize_occurrence_def(g, uri, container_prefix, parent_id, occ_node)
        }
        PBE::OccurrenceUsage(occ_node) => {
            usage_builders::materialize_occurrence_usage(
                occ_node,
                uri,
                container_prefix,
                parent_id,
                g,
            );
        }
        PBE::ConnectionDef(conn_node) => {
            materialize_connection_def(g, uri, container_prefix, parent_id, conn_node)
        }
        PBE::FlowDef(flow_node) => {
            materialize_flow_def(g, uri, container_prefix, parent_id, flow_node)
        }
        PBE::FlowUsage(flow_node) => {
            if let Some(parent_id) = parent_id {
                super::flow_usage::materialize_flow_usage(
                    flow_node,
                    uri,
                    container_prefix,
                    parent_id,
                    g,
                );
            }
        }
        PBE::AllocationDef(alloc_node) => {
            materialize_allocation_def(g, uri, container_prefix, parent_id, alloc_node)
        }
        PBE::Dependency(dep_node) => {
            materialize_dependency(g, uri, container_prefix, parent_id, dep_node)
        }
        PBE::ConstraintDef(c_node) => {
            calc_constraint_def::build_constraint_def(g, uri, container_prefix, parent_id, c_node);
        }
        PBE::ConstraintUsage(c_node) => {
            calc_constraint_def::build_constraint_usage(
                g,
                uri,
                container_prefix,
                parent_id,
                c_node,
            );
        }
        PBE::CalcDef(c_node) => {
            calc_constraint_def::build_calc_def(g, uri, container_prefix, parent_id, c_node);
        }
        PBE::CaseDef(c_node) => materialize_case_def(g, uri, container_prefix, parent_id, c_node),
        PBE::CaseUsage(c_node) => {
            materialize_case_usage(g, uri, container_prefix, parent_id, c_node)
        }
        PBE::AnalysisCaseDef(c_node) => {
            materialize_analysis_case_def(g, uri, container_prefix, parent_id, c_node)
        }
        PBE::AnalysisCaseUsage(c_node) => {
            materialize_analysis_case_usage(g, uri, container_prefix, parent_id, c_node)
        }
        PBE::VerificationCaseDef(c_node) => {
            materialize_verification_case_def(g, uri, container_prefix, parent_id, c_node)
        }
        PBE::VerificationCaseUsage(c_node) => {
            materialize_verification_case_usage(g, uri, container_prefix, parent_id, c_node)
        }
        PBE::Actor(actor_node) => {
            materialize_actor(g, uri, container_prefix, parent_id, actor_node)
        }
        PBE::StateDef(sd_node) => {
            materialize_state_def(g, uri, container_prefix, parent_id, sd_node)
        }
        PBE::StateUsage(su_node) => {
            materialize_state_usage(g, uri, container_prefix, parent_id, su_node)
        }
        PBE::ViewDef(vd_node) => {
            view_def::build_view_def(g, uri, container_prefix, parent_id, vd_node);
        }
        PBE::ViewpointDef(vpd_node) => {
            view_def::build_viewpoint_def(g, uri, container_prefix, parent_id, vpd_node);
        }
        PBE::RenderingDef(rd_node) => {
            view_def::build_rendering_def(g, uri, container_prefix, parent_id, rd_node);
        }
        PBE::ViewUsage(vu_node) => {
            view_def::build_view_usage(g, uri, container_prefix, parent_id, vu_node);
        }
        PBE::ViewpointUsage(vpu_node) => {
            view_def::build_viewpoint_usage(g, uri, container_prefix, parent_id, vpu_node);
        }
        PBE::RenderingUsage(ru_node) => {
            view_def::build_rendering_usage(g, uri, container_prefix, parent_id, ru_node);
        }
        PBE::Import(imp) => materialize_import(g, uri, container_prefix, parent_id, imp),
        // Intentionally omitted from the graph: parse placeholders and documentation-only members.
        PBE::Doc(doc) => {
            if let Some(pid) = parent_id {
                super::attach_doc_comment(g, pid, &doc.value.text);
            }
        }
        PBE::Error(_) | PBE::Comment(_) => {}
        PBE::TextualRep(t) => materialize_textual_rep(g, uri, container_prefix, parent_id, t),
        PBE::Filter(f) => {
            view_def::build_filter_member(g, uri, container_prefix, parent_id, f);
        }
        PBE::KermlSemanticDecl(k) => {
            kerml_library::build_kerml_semantic_decl(g, uri, container_prefix, parent_id, k);
        }
        PBE::KermlFeatureDecl(k) => {
            kerml_library::build_kerml_feature_decl(g, uri, container_prefix, parent_id, k);
        }
        PBE::ExtendedLibraryDecl(k) => {
            kerml_library::build_extended_library_decl(g, uri, container_prefix, parent_id, k);
        }
        PBE::Ref(r) => {
            if let Some(pid) = parent_id {
                super::ref_decl::materialize_ref_decl(
                    g,
                    uri,
                    container_prefix,
                    pid,
                    r,
                    super::ref_decl::RefDeclOptions::default(),
                );
            }
        }
        PBE::EnumerationUsage(enum_node) => {
            materialize_enum_usage(g, uri, container_prefix, parent_id, enum_node)
        }
        PBE::ItemUsage(_) | PBE::ConnectionUsage(_) | PBE::InterfaceUsage(_) => {}
    }
}
