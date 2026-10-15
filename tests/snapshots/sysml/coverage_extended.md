# META
~~~ini
description=Group 12: Extended Definitions and Usages (SysML §8.2.2.27)
type=file
~~~
# SOURCE
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure { part p; }
    #situation batteryLow;
    #situation x : T;
    #situation x : T { }
    variation #situation def V;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_extended.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 5) (end 1 14))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 2 4) (end 2 35))
        (related-information
          (related
            (uri "memory://snapshot/coverage_extended.md")
            (range (start 1 4) (end 1 27))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 5) (end 2 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 2 30) (end 2 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 14) (end 3 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 5) (end 4 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 22) (end 4 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 5) (end 5 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 29) (end 5 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 5) (end 6 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 5) (end 7 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 19) (end 7 20))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 8 4) (end 8 24))
        (related-information
          (related
            (uri "memory://snapshot/coverage_extended.md")
            (range (start 7 4) (end 7 21))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 5) (end 8 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 19) (end 8 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 15) (end 9 24))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a08a4585b019a2ab7c0ae953bc644b374fd65b6d7b320a7af1da016b9b29945d"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::AbstractFailure"))) (kind extended-definition) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "AbstractFailure")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "situation")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure"))))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 1))))) (kind extended-definition) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 2))))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "situation")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "situation")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 2)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "situation")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Failure::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::V"))) (kind extended-definition) (membership (kind owning) (visibility default)) (facts (modifiers variation) (implied-modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "V")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "situation")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Vulnerability"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Vulnerability")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "SecurityRelated")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Vulnerability")) (anonymous (kind metadata) (ordinal 1))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "situation")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::batteryLow"))) (kind extended-usage) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "batteryLow")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "situation")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x"))))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "T")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1))))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "T")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "situation")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "situation")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "AbstractFailure")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 1))))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 2)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "V")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Vulnerability")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "SecurityRelated")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Vulnerability")) (anonymous (kind metadata) (ordinal 1))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "batteryLow")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x"))))) (kind featureTyping) (ordinal 0))
      (authored-target "T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Failure::p"))) (target (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 2))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Failure::p")))
      (featured-by (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 2)))))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::AbstractFailure"))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure"))))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 1))))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 2))))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::V"))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Vulnerability"))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Vulnerability"))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::batteryLow"))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x"))))) (form prefix-keyword) (definition unresolved))
  (annotation (element (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1))))) (form prefix-keyword) (definition unresolved))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_extended.md") (range (start 3 14) (end 3 23)) (probe (position 3 14))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "AbstractFailure")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 2 30) (end 2 34)) (probe (position 2 30))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 1))))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 1 5) (end 1 14)) (probe (position 1 5))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 2 5) (end 2 14)) (probe (position 2 5))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 5 5) (end 5 14)) (probe (position 5 5))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 2)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 9 15) (end 9 24)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "V")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 4 5) (end 4 20)) (probe (position 4 5))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Vulnerability")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "SecurityRelated")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 4 22) (end 4 31)) (probe (position 4 22))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Vulnerability")) (anonymous (kind metadata) (ordinal 1))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 6 5) (end 6 14)) (probe (position 6 5))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "batteryLow")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 7 19) (end 7 20)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x"))))) (kind featureTyping) (ordinal 0) (authored-target "T")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 8 19) (end 8 20)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1))))) (kind featureTyping) (ordinal 0) (authored-target "T")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 7 5) (end 7 14)) (probe (position 7 5))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 8 5) (end 8 14)) (probe (position 8 5))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
)
~~~
