# META
~~~ini
description=Allocation typing and endpoints, and a package import filter that is not Boolean
type=file
~~~
# SOURCE
~~~sysml
package Allocations {
    part def Source;
    part def Target;
    allocation def Conforming;
    part source : Source;
    part target : Target;

    part def Allocating {
        allocation ok : Conforming;
        allocation wrong : Source;
        allocate source to target;
    }
}
package Filters {
    filter 1;
}
package BooleanFilter {
    filter true;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/allocation_and_filters.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 9 27) (end 9 33))
        (related-information
          (related
            (uri "memory://snapshot/allocation_and_filters.md")
            (range (start 1 4) (end 1 20))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "invalid_import_filter")
        (source "semantic")
        (range (start 14 11) (end 14 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2bb87fede3aa64ee6307bdda748fe54fbdfb9ab3a530b563ae6a7a52ca1688a0") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (path (named (kind package) (name "Allocations")) (named (kind part-def) (name "Allocating")) (anonymous (kind allocate) (ordinal 0))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "source")) (allocateTarget (reference "target")))))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::ok"))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Conforming")))))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::wrong"))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Source")))))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Conforming"))) (kind allocation-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Target"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::source"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Source")))))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::target"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Target")))))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "BooleanFilter"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Filters"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (path (named (kind package) (name "Allocations")) (named (kind part-def) (name "Allocating")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::source")))))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (path (named (kind package) (name "Allocations")) (named (kind part-def) (name "Allocating")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::target")))))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::ok"))) (kind featureTyping) (ordinal 0))
      (authored-target "Conforming")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Conforming")))))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::wrong"))) (kind featureTyping) (ordinal 0))
      (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")))))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")))))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Target")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Target")))))
  )
  (relationships
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/allocation_and_filters.md") (path (named (kind package) (name "Allocations")) (named (kind part-def) (name "Allocating")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_and_filters.md") (path (named (kind package) (name "Allocations")) (named (kind part-def) (name "Allocating")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/allocation_and_filters.md") (path (named (kind package) (name "Allocations")) (named (kind part-def) (name "Allocating")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_and_filters.md") (path (named (kind package) (name "Allocations")) (named (kind part-def) (name "Allocating")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::ok"))) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Conforming"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::ok"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::wrong"))) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::wrong"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::source"))) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::target"))) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::target"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "BooleanFilter"))) (form package-import) (state literal) (start 17 11) (end 17 15) (value (kind boolean) (boolean true)))
    (filter (owner (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Filters"))) (form package-import) (state literal) (start 14 11) (end 14 12) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (path (named (kind package) (name "Allocations")) (named (kind part-def) (name "Allocating")) (anonymous (kind allocate) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating")))
    )
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::ok")))
      (featured-by (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating")))
      (type (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Conforming")) (provenance authored))
      (effective-type (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Conforming")) (source direct))
      (supertype (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Conforming")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::wrong")))
      (featured-by (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating")))
      (type (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")) (provenance authored))
      (effective-type (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")) (source direct))
      (supertype (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Conforming")))
      (subtype (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::ok")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")))
      (subtype (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::wrong")) (scopes any))
      (subtype (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::source")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Target")))
      (subtype (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::source")))
      (type (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")) (provenance authored))
      (effective-type (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")) (source direct))
      (supertype (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::target")))
      (type (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Target")) (provenance authored))
      (effective-type (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Target")) (source direct))
      (supertype (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/allocation_and_filters.md") (range (start 10 17) (end 10 23)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (path (named (kind package) (name "Allocations")) (named (kind part-def) (name "Allocating")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::source")))))
    )
  )
  (query (document "memory://snapshot/allocation_and_filters.md") (range (start 10 27) (end 10 33)) (probe (position 10 27))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (path (named (kind package) (name "Allocations")) (named (kind part-def) (name "Allocating")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::target")))))
    )
  )
  (query (document "memory://snapshot/allocation_and_filters.md") (range (start 8 24) (end 8 34)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::ok"))) (kind featureTyping) (ordinal 0) (authored-target "Conforming")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Conforming")))))
    )
  )
  (query (document "memory://snapshot/allocation_and_filters.md") (range (start 9 27) (end 9 33)) (probe (position 9 27))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Allocating::wrong"))) (kind featureTyping) (ordinal 0) (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")))))
    )
  )
  (query (document "memory://snapshot/allocation_and_filters.md") (range (start 4 18) (end 4 24)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::source"))) (kind featureTyping) (ordinal 0) (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Source")))))
    )
  )
  (query (document "memory://snapshot/allocation_and_filters.md") (range (start 5 18) (end 5 24)) (probe (position 5 18))
    (reference (id (source (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::target"))) (kind featureTyping) (ordinal 0) (authored-target "Target")
      (outcome (status resolved) (target (node (document "memory://snapshot/allocation_and_filters.md") (qualified-name "Allocations::Target")))))
    )
  )
)
~~~
