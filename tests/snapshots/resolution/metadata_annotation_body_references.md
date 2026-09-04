# META
~~~ini
description=References inside a metadata annotation body (`@M { feat = value; }`) resolve against the annotated metadata definition: the redefined feature on the left of `=` and the value on the right
type=file
~~~
# SOURCE
~~~sysml
package MetadataAnnotationBody {
    enum def ConcernKind {
        purpose;
        structure;
    }

    attribute def Score {
        attribute weight : ConcernKind;
    }

    metadata def EngineeringConcern {
        attribute concern : ConcernKind;
        attribute score : Score;
    }

    // Conforming: `concern` binds to `EngineeringConcern::concern` (redefinition) and
    // `purpose` resolves against its type `ConcernKind`.
    part def Annotated {
        @EngineeringConcern { concern = purpose; }
    }

    // Conforming: a nested metadata body redefines a feature of the redefined feature's type.
    part def NestedAnnotated {
        @EngineeringConcern {
            concern = structure;
            score { weight = purpose; }
        }
    }

    // Negative control: `missing` is not a feature of `EngineeringConcern`, so the body
    // redefinition stays unresolved.
    part def BadAnnotated {
        @EngineeringConcern { missing = purpose; }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/metadata_annotation_body_references.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 30) (end 32 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 40) (end 32 47))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/metadata_annotation_body_references.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 30) (end 32 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 40) (end 32 47))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:46419bb9dcd79f52e32f33aa647e9508fa6d7ca2fbde6cb4d71a7b44d6cd431e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Annotated"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "EngineeringConcern")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "concern") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "concern")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "purpose")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::BadAnnotated"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "EngineeringConcern")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "missing")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "purpose")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::structure"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ConcernKind")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Score")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::NestedAnnotated"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "EngineeringConcern")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "concern") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "concern")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "score") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "score")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "structure")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "weight") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "weight")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "purpose")))))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ConcernKind")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "EngineeringConcern")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "concern")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "purpose")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "EngineeringConcern")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "missing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "purpose")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern"))) (kind featureTyping) (ordinal 0))
      (authored-target "ConcernKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score"))) (kind featureTyping) (ordinal 0))
      (authored-target "Score")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "EngineeringConcern")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "concern")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "score")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "weight")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::structure")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "purpose")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose")))))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight"))) (kind featureTyping) (ordinal 0))
      (authored-target "ConcernKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")))))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern"))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score"))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::structure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight"))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Annotated"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::BadAnnotated"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose"))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::structure"))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern"))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score"))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::NestedAnnotated"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::structure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight"))) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Annotated")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (source inherited) (from (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern"))))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (scopes any))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::BadAnnotated")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")))
      (subtype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern")) (scopes any))
      (subtype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose")))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")))
      (subtype (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::structure")))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")))
      (subtype (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern")))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern")))
      (type (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (source direct))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (scopes any))
      (subtype (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score")))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern")))
      (type (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score")) (provenance authored))
      (effective-type (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score")) (source direct))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score")) (scopes any))
      (subtype (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::NestedAnnotated")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (source inherited) (from (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern"))))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (scopes any))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score")) (source inherited) (from (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score"))))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score")) (scopes any feature))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (source inherited) (from (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight"))))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (scopes any))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::structure")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score")))
      (subtype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight")))
      (featured-by (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score")))
      (type (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (source direct))
      (supertype (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")) (scopes any))
      (subtype (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "purpose" (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose")))))
  (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "purpose" (target unresolved)))
  (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "structure" (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::structure")))))
  (declaration (id (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "purpose" (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 18 9) (end 18 27)) (probe (position 18 9))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "EngineeringConcern")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 18 30) (end 18 37)) (probe (position 18 30))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "concern")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 18 40) (end 18 47)) (probe (position 18 40))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "purpose")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 32 9) (end 32 27)) (probe (position 32 9))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "EngineeringConcern")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 32 30) (end 32 37)) (probe (position 32 30))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "missing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 32 40) (end 32 47)) (probe (position 32 40))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "BadAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "purpose")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 11 28) (end 11 39)) (probe (position 11 28))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern"))) (kind featureTyping) (ordinal 0) (authored-target "ConcernKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 12 26) (end 12 31)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score"))) (kind featureTyping) (ordinal 0) (authored-target "Score")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 23 9) (end 23 27)) (probe (position 23 9))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "EngineeringConcern")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 24 12) (end 24 19)) (probe (position 24 12))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "concern")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::concern")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 25 12) (end 25 17)) (probe (position 25 12))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "score")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::EngineeringConcern::score")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 25 20) (end 25 26)) (probe (position 25 20))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "weight")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 24 22) (end 24 31)) (probe (position 24 22))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::structure")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 25 29) (end 25 36)) (probe (position 25 29))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (path (named (kind package) (name "MetadataAnnotationBody")) (named (kind part-def) (name "NestedAnnotated")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "purpose")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind::purpose")))))
    )
  )
  (query (document "memory://snapshot/metadata_annotation_body_references.md") (range (start 7 27) (end 7 38)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::Score::weight"))) (kind featureTyping) (ordinal 0) (authored-target "ConcernKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_annotation_body_references.md") (qualified-name "MetadataAnnotationBody::ConcernKind")))))
    )
  )
)
~~~
