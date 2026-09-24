# META
~~~ini
description=Coverage: Metadata features with about clause and named metadata
type=file
~~~
# SOURCE
~~~sysml
metadata def Classified;
metadata def Approval;

package Annotated {
    @ Classified about Annotated;

    part def Vehicle;
    part def Engine;

    metadata m : Classified about Vehicle, Engine;

    #Classified part def AnnotatedPart;

    #Approval #Classified part def MultiAnnotated;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_metadata.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 11 4) (end 11 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 4) (end 13 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 14) (end 13 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:b6dea6c5b4a157636cd1481ba69322e85512b6e1cdab2e60932d7965b6c5b6c0"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Classified")) (metadataAnnotation (reference "Classified")) (metadataAnnotationAbout (reference "Annotated")))))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::AnnotatedPart"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::MultiAnnotated"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Classified")) (metadataAnnotationAbout (reference "Vehicle")) (metadataAnnotationAbout (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Approval"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified"))) (kind metadata-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Annotated")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated")))))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind metadataAnnotationAbout) (ordinal 1))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Engine")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind metadataAnnotationAbout) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)))))
      (type (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m")))
      (type (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))
      (subtype (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m")) (scopes any))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated"))) (form annotating-member) (definition (resolved (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))) (about (resolved (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated")))))
  (annotation (element (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Engine"))) (form usage) (definition (resolved (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))) (about (resolved (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Vehicle")))) (about (resolved (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Engine")))))
  (annotation (element (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Vehicle"))) (form usage) (definition (resolved (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))) (about (resolved (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Vehicle")))) (about (resolved (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Engine")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_metadata.md") (range (start 4 6) (end 4 16)) (probe (position 4 6))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))))
    )
  )
  (query (document "memory://snapshot/coverage_metadata.md") (range (start 4 6) (end 4 16)) (probe (position 4 6))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))))
    )
  )
  (query (document "memory://snapshot/coverage_metadata.md") (range (start 4 23) (end 4 32)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (path (named (kind package) (name "Annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Annotated")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated")))))
    )
  )
  (query (document "memory://snapshot/coverage_metadata.md") (range (start 9 17) (end 9 27)) (probe (position 9 17))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind featureTyping) (ordinal 0) (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))))
    )
  )
  (query (document "memory://snapshot/coverage_metadata.md") (range (start 9 34) (end 9 41)) (probe (position 9 34))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/coverage_metadata.md") (range (start 9 43) (end 9 49)) (probe (position 9 43))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind metadataAnnotationAbout) (ordinal 1) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Engine")))))
    )
  )
)
~~~
