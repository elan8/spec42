# META
~~~ini
description=Issue 297: retain extended members recovery
libraries=none
~~~
# SOURCE
~~~sysml
package ExtendedRecovery {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    part def P {
        #q def Bad : ;
        #q retained;
        #q broken : ;
        #q def Good;
        part later;
    }
    action def A {
        #q def Bad : { doc /* } ; */ }
        #q def Good;
    }
    enum def Choices {
        #q def Bad;
        enum retained;
    }
    dependency from A to P {
        #q def Bad;
        doc /* retained */
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_extended_members_recovery.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1 26) (end 1 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 3 8) (end 3 10))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 3 11) (end 4 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 5 8) (end 5 10))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 5 11) (end 6 8))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 8) (end 7 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 10 8) (end 10 10))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 10 11) (end 11 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 14 8) (end 15 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 18 8) (end 19 8))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_extended_members_recovery.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1 26) (end 1 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 3 8) (end 3 10))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 3 11) (end 4 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 5 8) (end 5 10))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 5 11) (end 6 8))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 8) (end 7 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 10 8) (end 10 10))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 10 11) (end 11 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 14 8) (end 15 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 18 8) (end 19 8))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation false) (source-digest "blake3:998eb3fd6489fd516a0d192cc3babb82795ef79fcf1bdbc08a72ffc76b0ebd5b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "A")) (dependencySupplier (reference "P")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "retained "))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::A::Good"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind action-def) (name "A")) (named (kind extended-definition) (name "Good")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Choices"))) (kind enum-def) (membership (kind owning) (visibility default)) (facts (implied-modifiers abstract variation)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Choices::retained"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P::Good"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind part-def) (name "P")) (named (kind extended-definition) (name "Good")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P::later"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P::retained"))) (kind extended-usage) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind part-def) (name "P")) (named (kind extended-usage) (name "retained")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "q")) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Metaobjects::SemanticMetadata")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::A")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind action-def) (name "A")) (named (kind extended-definition) (name "Good")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind part-def) (name "P")) (named (kind extended-definition) (name "Good")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind part-def) (name "P")) (named (kind extended-usage) (name "retained")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q"))) (kind specialization) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind action-def) (name "A")) (named (kind extended-definition) (name "Good")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind action-def) (name "A")) (named (kind extended-definition) (name "Good")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind part-def) (name "P")) (named (kind extended-definition) (name "Good")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind part-def) (name "P")) (named (kind extended-definition) (name "Good")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind part-def) (name "P")) (named (kind extended-usage) (name "retained")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind part-def) (name "P")) (named (kind extended-usage) (name "retained")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Choices::retained"))) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Choices"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P::later"))) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P::retained"))) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Choices::retained")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Choices")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P::later")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P::retained")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P")))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::A::Good"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P::Good"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P::retained"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parser_284_extended_members_recovery.md") (range (start 17 20) (end 17 21)) (probe (position 17 20))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::A")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_recovery.md") (range (start 17 25) (end 17 26)) (probe (position 17 25))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::P")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_recovery.md") (range (start 11 9) (end 11 10)) (probe (position 11 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind action-def) (name "A")) (named (kind extended-definition) (name "Good")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_recovery.md") (range (start 6 9) (end 6 10)) (probe (position 6 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind part-def) (name "P")) (named (kind extended-definition) (name "Good")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_recovery.md") (range (start 4 9) (end 4 10)) (probe (position 4 9))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (path (named (kind package) (name "ExtendedRecovery")) (named (kind part-def) (name "P")) (named (kind extended-usage) (name "retained")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_members_recovery.md") (range (start 1 26) (end 1 55)) (probe (position 1 26))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_members_recovery.md") (qualified-name "ExtendedRecovery::Q"))) (kind specialization) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
)
~~~
