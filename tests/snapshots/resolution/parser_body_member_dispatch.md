# META
~~~ini
description=Body-member kinds the 7d4fd85 parser added to nine body-element sets, each dispatched to its existing owner
type=file
~~~
# SOURCE
~~~sysml
package BodyMemberDispatch {
    part def P;
    item def I;
    attribute def A;
    connection def CD;
    viewpoint def VP;
    requirement def VR;

    part hostPart {
        end e1;
        in inParam : A;
    }

    occurrence def OD {
        ref item refInOccurrence : I;
        connection occConn : CD connect refInOccurrence to refInOccurrence;
    }

    port def PD {
        ref item refInPort : I;
    }

    requirement def R {
        ref part refInRequirement : P;
        concern nestedConcern;
        calc nestedCalc { 1 }
    }

    use case def U {
        use case nestedUseCase;
        case nestedCase;
        verification nestedVerification;
    }

    view def VD {
        ref part refInViewDef : P;
        viewpoint nestedViewpoint : VP;
        satisfy VR by hostPart;
    }

    view v : VD {
        ref part refInView : P;
    }

    rendering def RD {
        ref part refInRendering : P;
    }

    constraint def CReq {
        require constraint nestedRequire { 1 > 0 }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_body_member_dispatch.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 8 4) (end 11 5))
      )
      (diagnostic
        (severity warning)
        (code "viewpoint_conformance_invalid_target_kind")
        (source "semantic")
        (range (start 37 16) (end 37 18))
        (related-information
          (related
            (uri "memory://snapshot/parser_body_member_dispatch.md")
            (range (start 6 4) (end 6 23))
          )
        )
      )
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 40 4) (end 42 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:2704ca3d8324d26fc0d4d32c7c9e1a0ed5213433a4268011e4c88d88868e7b1b") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::A"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CD"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CReq"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CReq::nestedRequire"))) (kind require-constraint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CD")) (connectorEnd (reference "refInOccurrence")) (connectorEnd (reference "refInOccurrence")))))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "I")))))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD::refInPort"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "I")))))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::nestedCalc"))) (kind calc) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::nestedConcern"))) (kind concern) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::refInRequirement"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD::refInRendering"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U::nestedCase"))) (kind case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U::nestedUseCase"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U::nestedVerification"))) (kind verification) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "VR")) (satisfyTarget (reference "hostPart")))))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::nestedViewpoint"))) (kind viewpoint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VP")))))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::refInViewDef"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VP"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VR"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::e1"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::inParam"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A") (direction in)))))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VD")))))
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v::refInView"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (kind featureTyping) (ordinal 0))
      (authored-target "CD")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CD")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (kind connectorEnd) (ordinal 0))
      (authored-target "refInOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (kind connectorEnd) (ordinal 1))
      (authored-target "refInOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "I")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD::refInPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "I")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::refInRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD::refInRendering"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "VR")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VR")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0))
      (authored-target "hostPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::nestedViewpoint"))) (kind featureTyping) (ordinal 0))
      (authored-target "VP")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VP")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::refInViewDef"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::inParam"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::A")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "VD")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD")))))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v::refInView"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD::refInPort"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD::refInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::refInRequirement"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::refInRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD::refInRendering"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD::refInRendering"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VR"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::nestedViewpoint"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VP"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::nestedViewpoint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::refInViewDef"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::refInViewDef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::inParam"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::inParam"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v::refInView"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v::refInView"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CReq::nestedRequire"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CReq"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD::refInPort"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::nestedCalc"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::nestedConcern"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::refInRequirement"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD::refInRendering"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U::nestedCase"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U::nestedUseCase"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U::nestedVerification"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::nestedViewpoint"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::refInViewDef"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::e1"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::inParam"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v::refInView"))) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CReq::nestedRequire"))) (state evaluated) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::nestedCalc"))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::A")))
      (subtype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::inParam")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CD")))
      (subtype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CReq::nestedRequire")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CReq")))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")))
      (subtype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD::refInPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD")))
      (type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CD")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CD")) (source direct))
      (supertype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CD")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD")))
      (type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")) (source direct))
      (supertype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")))
      (subtype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::refInRequirement")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD::refInRendering")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::refInViewDef")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v::refInView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD::refInPort")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD")))
      (type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")) (source direct))
      (supertype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::nestedCalc")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R")))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::nestedConcern")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R")))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::refInRequirement")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R")))
      (type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (source direct))
      (supertype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD::refInRendering")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD")))
      (type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (source direct))
      (supertype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U::nestedCase")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U")))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U::nestedUseCase")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U")))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U::nestedVerification")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::U")))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD")))
      (subtype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD")))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::nestedViewpoint")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD")))
      (type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VP")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VP")) (source direct))
      (supertype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VP")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::refInViewDef")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD")))
      (type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (source direct))
      (supertype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VP")))
      (subtype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::nestedViewpoint")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart")))
      (positional-ends (authored 1) (effective 1))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::e1")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart")))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::inParam")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart")))
      (type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::A")) (source direct))
      (supertype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v")))
      (type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD")) (source direct))
      (supertype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v::refInView")))
      (featured-by (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v")))
      (type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (source direct))
      (supertype (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 15 29) (end 15 31)) (probe (position 15 29))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (kind featureTyping) (ordinal 0) (authored-target "CD")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::CD")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 15 40) (end 15 55)) (probe (position 15 40))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (kind connectorEnd) (ordinal 0) (authored-target "refInOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 15 59) (end 15 74)) (probe (position 15 59))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::occConn"))) (kind connectorEnd) (ordinal 1) (authored-target "refInOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 14 35) (end 14 36)) (probe (position 14 35))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::OD::refInOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "I")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 19 29) (end 19 30)) (probe (position 19 29))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::PD::refInPort"))) (kind featureTyping) (ordinal 0) (authored-target "I")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::I")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 23 36) (end 23 37)) (probe (position 23 36))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::R::refInRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 45 34) (end 45 35)) (probe (position 45 34))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::RD::refInRendering"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 37 16) (end 37 18)) (probe (position 37 16))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "VR")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VR")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 37 22) (end 37 30)) (probe (position 37 22))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (path (named (kind package) (name "BodyMemberDispatch")) (named (kind view-def) (name "VD")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0) (authored-target "hostPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 36 36) (end 36 38)) (probe (position 36 36))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::nestedViewpoint"))) (kind featureTyping) (ordinal 0) (authored-target "VP")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VP")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 35 32) (end 35 33)) (probe (position 35 32))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD::refInViewDef"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 10 21) (end 10 22)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::hostPart::inParam"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::A")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 40 13) (end 40 15)) (probe (position 40 13))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v"))) (kind featureTyping) (ordinal 0) (authored-target "VD")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::VD")))))
    )
  )
  (query (document "memory://snapshot/parser_body_member_dispatch.md") (range (start 41 29) (end 41 30)) (probe (position 41 29))
    (reference (id (source (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::v::refInView"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_body_member_dispatch.md") (qualified-name "BodyMemberDispatch::P")))))
    )
  )
)
~~~
