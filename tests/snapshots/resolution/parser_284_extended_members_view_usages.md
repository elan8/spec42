# META
~~~ini
description=Issue 297: retain extended members view usages
libraries=none
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package ExtendedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    view v {
        #q def Inner;
        #q instance : Inner;
    }
    viewpoint vp {
        #q def Inner;
        #q instance : Inner;
    }
    rendering r {
        #q def Inner;
        #q instance : Inner;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_extended_members_view_usages.md"
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
  (relationship (kind feature_typing) (source "ExtendedOwners::v::instance") (target "ExtendedOwners::v::Inner") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "ExtendedOwners::vp::instance") (target "ExtendedOwners::vp::Inner") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "ExtendedOwners::r::instance") (target "ExtendedOwners::r::Inner") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_extended_members_view_usages.md"
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ca4502e0a7ddcd8d6aef999a60dc9c82ce55f319e4105482f59447c5ded455c6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "q")) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Metaobjects::SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r"))) (kind rendering) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering) (name "r")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering) (name "r")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view) (name "v")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view) (name "v")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp"))) (kind viewpoint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint) (name "vp")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint) (name "vp")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q"))) (kind specialization) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering) (name "r")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering) (name "r")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view) (name "v")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view) (name "v")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint) (name "vp")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint) (name "vp")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering) (name "r")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering) (name "r")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering) (name "r")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering) (name "r")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view) (name "v")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view) (name "v")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view) (name "v")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view) (name "v")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint) (name "vp")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint) (name "vp")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint) (name "vp")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint) (name "vp")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r")))
      (type (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::Inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v")))
      (type (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::Inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp")))
      (type (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::Inner")) (scopes any))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parser_284_extended_members_view_usages.md") (range (start 1 26) (end 1 55)) (probe (position 1 26))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q"))) (kind specialization) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_usages.md") (range (start 11 9) (end 11 10)) (probe (position 11 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering) (name "r")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_usages.md") (range (start 12 22) (end 12 27)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::r::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_usages.md") (range (start 12 9) (end 12 10)) (probe (position 12 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering) (name "r")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_usages.md") (range (start 3 9) (end 3 10)) (probe (position 3 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view) (name "v")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_usages.md") (range (start 4 22) (end 4 27)) (probe (position 4 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::v::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_usages.md") (range (start 4 9) (end 4 10)) (probe (position 4 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view) (name "v")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_usages.md") (range (start 7 9) (end 7 10)) (probe (position 7 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint) (name "vp")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_usages.md") (range (start 8 22) (end 8 27)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::vp::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_usages.md") (range (start 8 9) (end 8 10)) (probe (position 8 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint) (name "vp")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_usages.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
)
~~~
