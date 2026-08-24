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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:b6dea6c5b4a157636cd1481ba69322e85512b6e1cdab2e60932d7965b6c5b6c0") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated"))) (kind package) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Classified")))))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::AnnotatedPart"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::MultiAnnotated"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Classified")))))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Approval"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified"))) (kind metadata-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated"))) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m")))
      (type (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))
      (subtype (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_metadata.md") (range (start 4 6) (end 4 16)) (probe (position 4 6))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))))
    )
  )
  (query (document "memory://snapshot/coverage_metadata.md") (range (start 9 17) (end 9 27)) (probe (position 9 17))
    (reference (id (source (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Annotated::m"))) (kind featureTyping) (ordinal 0) (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_metadata.md") (qualified-name "Classified")))))
    )
  )
)
~~~
