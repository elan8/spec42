# META
~~~ini
description=Issue 297: retain extended members behavior definitions
libraries=none
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package ExtendedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    action def A {
        #q def Inner;
        #q instance : Inner;
    }
    state def S {
        #q def Inner;
        #q instance : Inner;
    }
    calc def C {
        #q def Inner;
        #q instance : Inner;
    }
    constraint def Check {
        #q def Inner;
        #q instance : Inner;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1 26) (end 1 55))
      )
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind feature_typing) (source "ExtendedOwners::A::instance") (target "ExtendedOwners::A::Inner") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "ExtendedOwners::S::instance") (target "ExtendedOwners::S::Inner") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "ExtendedOwners::C::instance") (target "ExtendedOwners::C::Inner") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "ExtendedOwners::Check::instance") (target "ExtendedOwners::Check::Inner") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1 26) (end 1 55))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4eb3aca8659389ae1ceac4c5e3c0272836e335b538284c9d39ca515d2876feb3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind action-def) (name "A")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind action-def) (name "A")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind calc-def) (name "C")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind calc-def) (name "C")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind constraint-def) (name "Check")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind constraint-def) (name "Check")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "q")) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Metaobjects::SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind state-def) (name "S")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind state-def) (name "S")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind action-def) (name "A")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind action-def) (name "A")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind calc-def) (name "C")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind calc-def) (name "C")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind constraint-def) (name "Check")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind constraint-def) (name "Check")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (kind specialization) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind state-def) (name "S")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind state-def) (name "S")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind action-def) (name "A")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind action-def) (name "A")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind action-def) (name "A")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind action-def) (name "A")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind calc-def) (name "C")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind calc-def) (name "C")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind calc-def) (name "C")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind calc-def) (name "C")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind constraint-def) (name "Check")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind constraint-def) (name "Check")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind constraint-def) (name "Check")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind constraint-def) (name "Check")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind state-def) (name "S")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind state-def) (name "S")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind state-def) (name "S")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind state-def) (name "S")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A")))
      (type (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::Inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C")))
      (type (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::Inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check")))
      (type (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::Inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S")))
      (type (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::Inner")) (scopes any))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 3 9) (end 3 10)) (probe (position 3 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind action-def) (name "A")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 4 22) (end 4 27)) (probe (position 4 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::A::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 4 9) (end 4 10)) (probe (position 4 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind action-def) (name "A")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 11 9) (end 11 10)) (probe (position 11 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind calc-def) (name "C")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 12 22) (end 12 27)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::C::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 12 9) (end 12 10)) (probe (position 12 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind calc-def) (name "C")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 15 9) (end 15 10)) (probe (position 15 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind constraint-def) (name "Check")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 16 22) (end 16 27)) (probe (position 16 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Check::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 16 9) (end 16 10)) (probe (position 16 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind constraint-def) (name "Check")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 1 26) (end 1 55)) (probe (position 1 26))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q"))) (kind specialization) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 7 9) (end 7 10)) (probe (position 7 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind state-def) (name "S")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 8 22) (end 8 27)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::S::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (range (start 8 9) (end 8 10)) (probe (position 8 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind state-def) (name "S")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_behavior_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
)
~~~
