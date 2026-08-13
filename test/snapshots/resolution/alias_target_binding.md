# META
~~~ini
description=Alias target and binding resolution coverage
type=file
observed_gap=The alias declaration is published as an element, but its authored target is not exposed as a semantic reference; typing through the alias remains visible.
~~~
# SOURCE
~~~sysml
package AliasCoverage {
    part def Device;
    alias DeviceAlias for Device;
    part device : DeviceAlias;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/alias_target_binding.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 4) (end 2 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 18) (end 3 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:0bfd7ddd64d09313a48eb45faaf8d8009f0991ec3c0bb996b5cd24404ac9775a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::Device"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DeviceAlias"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device"))) (kind featureTyping) (ordinal 0))
      (authored-target "DeviceAlias")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/alias_target_binding.md") (range (start 3 18) (end 3 29)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device"))) (kind featureTyping) (ordinal 0) (authored-target "DeviceAlias")
      (outcome (status unresolved)))
  )
)
~~~
