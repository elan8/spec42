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

    // A prefix before any other member annotates that member, in a requirement definition
    // body and in an action definition body alike.
    requirement def ReqC {
        #refinement attribute note;
    }
    action def ActE {
        #refinement action step;
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
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 19) (end 1 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 40) (end 26 51))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:04f58974884ab00c177891b98727ade31cb60c94c79d83a58edcf8fbada76af3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Upstream") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ActE"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ActE::step"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind action-def) (name "ActE")) (named (kind action) (name "step")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartA"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartB"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "PartA")) (dependencyClient (reference "PartB")) (dependencySupplier (reference "Upstream::Target")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartD"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "PartD")) (dependencySupplier (reference "Nonexistent")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqB"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "ReqB")) (dependencySupplier (reference "Upstream::Target")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 1))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "trace")))))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC::note"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqC")) (named (kind attribute) (name "note")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::Target"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::trace"))) (kind metadata-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Upstream")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream")))))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind action-def) (name "ActE")) (named (kind action) (name "step")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
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
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqC")) (named (kind attribute) (name "note")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind action-def) (name "ActE")) (named (kind action) (name "step")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind action-def) (name "ActE")) (named (kind action) (name "step")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
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
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqC")) (named (kind attribute) (name "note")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqC")) (named (kind attribute) (name "note")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ActE::step"))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ActE"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartD"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC::note"))) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ActE::step")))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ActE")))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartB")))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartD")))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqB")))
    )
    (declaration (id (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC::note")))
      (featured-by (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC")))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ActE::step"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
  (annotation (element (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
  (annotation (element (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
  (annotation (element (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
  (annotation (element (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::trace")))))
  (annotation (element (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::ReqC::note"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
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
  (query (document "memory://snapshot/consumer.sysml") (range (start 21 9) (end 21 19)) (probe (position 21 9))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind action-def) (name "ActE")) (named (kind action) (name "step")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
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
  (query (document "memory://snapshot/consumer.sysml") (range (start 26 31) (end 26 36)) (probe (position 26 31))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "PartD")
      (outcome (status resolved) (target (node (document "memory://snapshot/consumer.sysml") (qualified-name "MetadataPrefixedDependencyEdges::PartD")))))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 26 40) (end 26 51)) (probe (position 26 40))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind part-def) (name "PartD")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "Nonexistent")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/consumer.sysml") (range (start 26 9) (end 26 19)) (probe (position 26 9))
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
  (query (document "memory://snapshot/consumer.sysml") (range (start 18 9) (end 18 19)) (probe (position 18 9))
    (reference (id (source (node (document "memory://snapshot/consumer.sysml") (path (named (kind package) (name "MetadataPrefixedDependencyEdges")) (named (kind requirement-def) (name "ReqC")) (named (kind attribute) (name "note")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/upstream.sysml") (qualified-name "Upstream::refinement")))))
    )
  )
)
~~~
