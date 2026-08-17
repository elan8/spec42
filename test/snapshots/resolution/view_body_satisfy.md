# META
~~~ini
description=A view body reaches the one satisfy production, both sides of the rule
type=file
~~~
# SOURCE
~~~sysml
package ViewCoverage {
    viewpoint def ArchitectureViewpoint;
    view def ArchitectureView;

    // A view body reaches SatisfyRequirementUsage the same way every other body does, so this
    // lowers through the one satisfy owner: an anonymous satisfy declaration owned by the view,
    // carrying a satisfySource reference. There is no view-specific satisfy shape and no
    // view-specific reference kind.
    view architecture : ArchitectureView {
        satisfy ArchitectureViewpoint;
    }

    // The other side of the rule. Nothing named MissingViewpoint is in scope, so the reference
    // stays explicitly unresolved rather than being fabricated or silently dropped.
    view unresolvable : ArchitectureView {
        satisfy MissingViewpoint;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/view_body_satisfy.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 16) (end 15 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:62eb545c688c3dc9349d84f36683c414050bca54e93ee67b87be17809075b115") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureViewpoint"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ArchitectureView")))))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (path (named (kind package) (name "ViewCoverage")) (named (kind view) (name "architecture")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "ArchitectureViewpoint")))))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::unresolvable"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ArchitectureView")))))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (path (named (kind package) (name "ViewCoverage")) (named (kind view) (name "unresolvable")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "MissingViewpoint")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind featureTyping) (ordinal 0))
      (authored-target "ArchitectureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")))))
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (path (named (kind package) (name "ViewCoverage")) (named (kind view) (name "architecture")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "ArchitectureViewpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureViewpoint")))))
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::unresolvable"))) (kind featureTyping) (ordinal 0))
      (authored-target "ArchitectureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")))))
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (path (named (kind package) (name "ViewCoverage")) (named (kind view) (name "unresolvable")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "MissingViewpoint")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/view_body_satisfy.md") (path (named (kind package) (name "ViewCoverage")) (named (kind view) (name "architecture")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureViewpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_body_satisfy.md") (path (named (kind package) (name "ViewCoverage")) (named (kind view) (name "architecture")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::unresolvable"))) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::unresolvable"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")))
      (subtype (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture")) (scopes any))
      (subtype (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::unresolvable")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture")))
      (type (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")) (source direct))
      (supertype (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (path (named (kind package) (name "ViewCoverage")) (named (kind view) (name "architecture")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture")))
    )
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::unresolvable")))
      (type (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")) (source direct))
      (supertype (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (path (named (kind package) (name "ViewCoverage")) (named (kind view) (name "unresolvable")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::unresolvable")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/view_body_satisfy.md") (range (start 8 24) (end 8 40)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind featureTyping) (ordinal 0) (authored-target "ArchitectureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")))))
    )
  )
  (query (document "memory://snapshot/view_body_satisfy.md") (range (start 9 16) (end 9 37)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (path (named (kind package) (name "ViewCoverage")) (named (kind view) (name "architecture")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "ArchitectureViewpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureViewpoint")))))
    )
  )
  (query (document "memory://snapshot/view_body_satisfy.md") (range (start 14 24) (end 14 40)) (probe (position 14 24))
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::unresolvable"))) (kind featureTyping) (ordinal 0) (authored-target "ArchitectureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")))))
    )
  )
  (query (document "memory://snapshot/view_body_satisfy.md") (range (start 15 16) (end 15 32)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (path (named (kind package) (name "ViewCoverage")) (named (kind view) (name "unresolvable")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "MissingViewpoint")
      (outcome (status unresolved)))
    )
  )
)
~~~
