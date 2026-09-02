# META
~~~ini
description=A metadata-prefixed dependency (`#refinement dependency X to Y;`) in a requirement, action, or part definition body lowers as a dependency relationship whose `#refinement` tag is an authored metadata annotation
type=file
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package MetadataPrefixedDependency {
    metadata def refinement;

    requirement def ReqA;
    requirement def ReqB {
        #refinement dependency ReqB to MetadataPrefixedDependency::ReqA;
    }

    action def ActA;
    action def ActB {
        #refinement dependency ActB to ActA;
    }

    part def PartA;
    part def PartB {
        #refinement dependency PartB to PartA;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/metadata_prefixed_dependency.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/metadata_prefixed_dependency.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:890d3dbe702f9a2377a29598f768dde54370eda753093684afb078de4f6d6ade"))
  (declarations
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ActA"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ActB"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "ActB")) (dependencySupplier (reference "ActA")))))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::PartA"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::PartB"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "PartB")) (dependencySupplier (reference "PartA")))))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ReqA"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ReqB"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "ReqB")) (dependencySupplier (reference "MetadataPrefixedDependency::ReqA")))))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "refinement")))))
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::refinement"))) (kind metadata-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "ActB")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ActB")))))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "ActA")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ActA")))))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::refinement")))))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "PartB")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::PartB")))))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "PartA")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::PartA")))))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::refinement")))))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "ReqB")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ReqB")))))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "MetadataPrefixedDependency::ReqA")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ReqA")))))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::refinement")))))
  )
  (relationships
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ActB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ActA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::refinement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::PartB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::PartA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::refinement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ReqB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ReqA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::refinement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ActB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::PartB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ReqB"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ActB")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::PartB")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ReqB")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/metadata_prefixed_dependency.md") (range (start 10 31) (end 10 35)) (probe (position 10 31))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "ActB")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ActB")))))
    )
  )
  (query (document "memory://snapshot/metadata_prefixed_dependency.md") (range (start 10 39) (end 10 43)) (probe (position 10 39))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "ActA")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ActA")))))
    )
  )
  (query (document "memory://snapshot/metadata_prefixed_dependency.md") (range (start 10 9) (end 10 19)) (probe (position 10 9))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind action-def) (name "ActB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::refinement")))))
    )
  )
  (query (document "memory://snapshot/metadata_prefixed_dependency.md") (range (start 15 31) (end 15 36)) (probe (position 15 31))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "PartB")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::PartB")))))
    )
  )
  (query (document "memory://snapshot/metadata_prefixed_dependency.md") (range (start 15 40) (end 15 45)) (probe (position 15 40))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "PartA")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::PartA")))))
    )
  )
  (query (document "memory://snapshot/metadata_prefixed_dependency.md") (range (start 15 9) (end 15 19)) (probe (position 15 9))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind part-def) (name "PartB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::refinement")))))
    )
  )
  (query (document "memory://snapshot/metadata_prefixed_dependency.md") (range (start 5 31) (end 5 35)) (probe (position 5 31))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "ReqB")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ReqB")))))
    )
  )
  (query (document "memory://snapshot/metadata_prefixed_dependency.md") (range (start 5 39) (end 5 71)) (probe (position 5 39))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "MetadataPrefixedDependency::ReqA")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::ReqA")))))
    )
  )
  (query (document "memory://snapshot/metadata_prefixed_dependency.md") (range (start 5 9) (end 5 19)) (probe (position 5 9))
    (reference (id (source (node (document "memory://snapshot/metadata_prefixed_dependency.md") (path (named (kind package) (name "MetadataPrefixedDependency")) (named (kind requirement-def) (name "ReqB")) (anonymous (kind dependency) (ordinal 0)) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "refinement")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_prefixed_dependency.md") (qualified-name "MetadataPrefixedDependency::refinement")))))
    )
  )
)
~~~
