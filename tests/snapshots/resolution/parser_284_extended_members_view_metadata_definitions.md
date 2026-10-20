# META
~~~ini
description=Issue 297: retain extended members view metadata definitions
libraries=none
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package ExtendedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    view def V {
        #q def Inner;
        #q instance : Inner;
    }
    viewpoint def VP {
        #q def Inner;
        #q instance : Inner;
    }
    rendering def R {
        #q def Inner;
        #q instance : Inner;
    }
    metadata def M {
        #q def Inner;
        #q instance : Inner;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md"
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
  (relationship (kind feature_typing) (source "ExtendedOwners::V::instance") (target "ExtendedOwners::V::Inner") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "ExtendedOwners::VP::instance") (target "ExtendedOwners::VP::Inner") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "ExtendedOwners::R::instance") (target "ExtendedOwners::R::Inner") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "ExtendedOwners::M::instance") (target "ExtendedOwners::M::Inner") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md"
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b516107f96a512fd563487e1a66a319839c2dca79bad09295ab118842be2486f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind metadata-def) (name "M")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind metadata-def) (name "M")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "q")) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Metaobjects::SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering-def) (name "R")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering-def) (name "R")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view-def) (name "V")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view-def) (name "V")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::Inner"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint-def) (name "VP")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::instance"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Inner")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint-def) (name "VP")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind metadata-def) (name "M")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind metadata-def) (name "M")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (kind specialization) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering-def) (name "R")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering-def) (name "R")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view-def) (name "V")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view-def) (name "V")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint-def) (name "VP")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::instance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::Inner")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint-def) (name "VP")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind metadata-def) (name "M")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind metadata-def) (name "M")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind metadata-def) (name "M")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind metadata-def) (name "M")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering-def) (name "R")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering-def) (name "R")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering-def) (name "R")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering-def) (name "R")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view-def) (name "V")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view-def) (name "V")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view-def) (name "V")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view-def) (name "V")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint-def) (name "VP")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint-def) (name "VP")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::Inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::instance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint-def) (name "VP")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint-def) (name "VP")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::instance"))) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M")))
      (type (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::Inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R")))
      (type (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::Inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V")))
      (type (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::Inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::Inner")))
      (subtype (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::instance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::instance")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP")))
      (type (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::Inner")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::Inner")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::Inner")) (scopes any))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::Inner"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::instance"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 15 9) (end 15 10)) (probe (position 15 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind metadata-def) (name "M")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 16 22) (end 16 27)) (probe (position 16 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::M::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 16 9) (end 16 10)) (probe (position 16 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind metadata-def) (name "M")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 1 26) (end 1 55)) (probe (position 1 26))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q"))) (kind specialization) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 11 9) (end 11 10)) (probe (position 11 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering-def) (name "R")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 12 22) (end 12 27)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::R::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 12 9) (end 12 10)) (probe (position 12 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind rendering-def) (name "R")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 3 9) (end 3 10)) (probe (position 3 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view-def) (name "V")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 4 22) (end 4 27)) (probe (position 4 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::V::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 4 9) (end 4 10)) (probe (position 4 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind view-def) (name "V")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 7 9) (end 7 10)) (probe (position 7 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint-def) (name "VP")) (named (kind extended-definition) (name "Inner")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 8 22) (end 8 27)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::instance"))) (kind featureTyping) (ordinal 0) (authored-target "Inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::VP::Inner")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (range (start 8 9) (end 8 10)) (probe (position 8 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (path (named (kind package) (name "ExtendedOwners")) (named (kind viewpoint-def) (name "VP")) (named (kind extended-usage) (name "instance")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_view_metadata_definitions.md") (qualified-name "ExtendedOwners::Q")))))
    )
  )
)
~~~
