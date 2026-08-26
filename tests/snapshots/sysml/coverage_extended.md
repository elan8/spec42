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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 2 30) (end 2 34))
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5a5054966a4ceb012f28acf6959cd39d3f34ebe2ff4dcc8d8f5effb32dc90714") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::AbstractFailure"))) (kind extended-definition) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure"))))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 1))))) (kind extended-definition) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 2))))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Failure::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::V"))) (kind extended-definition) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Vulnerability"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::batteryLow"))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "situation")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x"))))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "T")) (metadataAnnotation (reference "situation")))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1))))) (kind extended-usage) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "T")) (metadataAnnotation (reference "situation")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 1))))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::batteryLow"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x"))))) (kind featureTyping) (ordinal 0))
      (authored-target "T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x"))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1))))) (kind metadataAnnotation) (ordinal 0))
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
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_extended.md") (range (start 2 30) (end 2 34)) (probe (position 2 30))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-definition) (name "Failure") (occurrence 1))))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 6 5) (end 6 14)) (probe (position 6 5))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::batteryLow"))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
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
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x"))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_extended.md") (range (start 8 5) (end 8 14)) (probe (position 8 5))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (path (named (kind package) (name "ExtendedExamples")) (named (kind extended-usage) (name "x") (occurrence 1))))) (kind metadataAnnotation) (ordinal 0) (authored-target "situation")
      (outcome (status unresolved)))
    )
  )
)
~~~
