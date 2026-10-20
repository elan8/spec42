# META
~~~ini
description=Issue 297: action and calculation bodies retain nested structure/view definitions and every case usage; state and use-case bodies retain variants
libraries=none
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package Bodies284 {
    part def P;
    item def I;
    action def A {
        part def NestedPart;
        item def NestedItem;
        view def NestedView;
        rendering def NestedRendering;
        viewpoint def NestedViewpoint;
        case c;
        analysis a;
        verification v;
        use case u;
    }
    calc def C {
        part def NestedPart;
        item def NestedItem;
        view def NestedView;
        rendering def NestedRendering;
        viewpoint def NestedViewpoint;
        case c;
        analysis a;
        verification v;
        use case u;
    }
    variation use case choices {
        variant use case chosen;
        variant case generic;
        variant analysis analyzed;
        variant verification verified;
    }
    variation state def States {
        variant state on;
        variant state off;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_action_body_members.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_action_body_members.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:88b5ca5c3a293392f1e2522011ba2cffc52b7767cc2f6d7df71ec63eab2e877c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::NestedItem"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::NestedPart"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::NestedRendering"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::NestedView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::NestedViewpoint"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::a"))) (kind analysis) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::c"))) (kind case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::u"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::v"))) (kind verification) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::NestedItem"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::NestedPart"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::NestedRendering"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::NestedView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::NestedViewpoint"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::a"))) (kind analysis) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::c"))) (kind case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::u"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::v"))) (kind verification) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::I"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::States"))) (kind state-def) (membership (kind owning) (visibility default)) (facts (modifiers variation) (implied-modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::States::off"))) (kind state) (membership (kind owning) (visibility default) (role variant)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::States::on"))) (kind state) (membership (kind owning) (visibility default) (role variant)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::choices"))) (kind use-case) (membership (kind feature) (visibility default)) (facts (modifiers variation) (implied-modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::choices::analyzed"))) (kind analysis) (membership (kind owning) (visibility default) (role variant)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::choices::chosen"))) (kind use-case) (membership (kind owning) (visibility default) (role variant)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::choices::generic"))) (kind case) (membership (kind owning) (visibility default) (role variant)))
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::choices::verified"))) (kind verification) (membership (kind owning) (visibility default) (role variant)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::a"))) (target (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::c"))) (target (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::u"))) (target (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::v"))) (target (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::a"))) (target (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::c"))) (target (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::u"))) (target (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::v"))) (target (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::a")))
      (featured-by (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::c")))
      (featured-by (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::u")))
      (featured-by (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A::v")))
      (featured-by (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::A")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::a")))
      (featured-by (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::c")))
      (featured-by (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::u")))
      (featured-by (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C::v")))
      (featured-by (node (document "memory://snapshot/parser_284_action_body_members.md") (qualified-name "Bodies284::C")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
