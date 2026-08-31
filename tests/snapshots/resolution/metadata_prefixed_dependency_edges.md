# META
~~~ini
description=Metadata-prefixed dependency edge cases -- stacked prefixes, cross-document and multiple endpoints, a prefix before a non-dependency member, and an unresolved supplier
type=file
~~~
# SOURCE
## upstream.sysml
~~~sysml
package Upstream {
    metadata def refinement;
    metadata def trace;
    part def Target;
}
~~~
## consumer.sysml
~~~sysml
package MetadataPrefixedDependencyEdges {
    private import Upstream::*;

    part def PartA;

    // Multiple clients and a cross-document qualified supplier.
    part def PartB {
        #refinement dependency from PartA, PartB to Upstream::Target;
    }

    // Stacked prefixes bind to the same dependency.
    requirement def ReqB {
        #refinement #trace dependency ReqB to Upstream::Target;
    }

    // A prefix before a non-dependency member stays an explicit unsupported member.
    requirement def ReqC {
        #refinement attribute note;
    }

    // The dependency and its #refinement annotation still lower when a supplier is unresolved.
    part def PartD {
        #refinement dependency PartD to Nonexistent;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/consumer.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 19) (end 1 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 17 8) (end 17 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 40) (end 22 51))
      )
    )
  )
  (document "memory://snapshot/upstream.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:651167141b581fccf1ccd52225638a12f47849400892cf0885dad5ad9065f96c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Upstream") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartA"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartB"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "PartA")) (dependencyClient (reference "PartB")) (dependencySupplier (reference "Upstream::Target")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartD"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "PartD")) (dependencySupplier (reference "Nonexistent")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqB"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "ReqB")) (dependencySupplier (reference "Upstream::Target")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 1))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "trace")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC::note"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::Target"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::trace"))) (kind metadata-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Upstream")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "PartA")
      (outcome (status resolved) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartA")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 1))
      (authored-target "PartB")
      (outcome (status resolved) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartB")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Upstream::Target")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::Target")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "PartD")
      (outcome (status resolved) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartD")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Nonexistent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "ReqB")
      (outcome (status resolved) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqB")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Upstream::Target")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::Target")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 1))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "trace")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::trace")))))
  )
  (relationships
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 1)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::Target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartD"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::Target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 1))))) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::trace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 1))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 1))))) (target (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC::note"))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartB")))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartD")))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqB")))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC::note")))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/consumer.sysml") (range (start 1 19) (end 1 30)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Upstream")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 7 36) (end 7 41)) (probe (position 7 36))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "PartA")
      (outcome (status resolved) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartA")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 7 43) (end 7 48)) (probe (position 7 43))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 1) (authored-target "PartB")
      (outcome (status resolved) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartB")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 7 52) (end 7 68)) (probe (position 7 52))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "Upstream::Target")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::Target")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 7 9) (end 7 19)) (probe (position 7 9))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 22 31) (end 22 36)) (probe (position 22 31))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "PartD")
      (outcome (status resolved) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartD")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 22 40) (end 22 51)) (probe (position 22 40))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "Nonexistent")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 22 9) (end 22 19)) (probe (position 22 9))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 12 38) (end 12 42)) (probe (position 12 38))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "ReqB")
      (outcome (status resolved) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqB")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 12 46) (end 12 62)) (probe (position 12 46))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "Upstream::Target")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::Target")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 12 9) (end 12 19)) (probe (position 12 9))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 12 21) (end 12 26)) (probe (position 12 21))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 1))))) (kind metadataAnnotation) (ordinal 0) (authored-target "trace")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::trace")))))
    )
  )
)
~~~
