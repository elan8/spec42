# META
~~~ini
description=Alias target and binding resolution coverage
type=file
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0bfd7ddd64d09313a48eb45faaf8d8009f0991ec3c0bb996b5cd24404ac9775a"))
  (declarations
    (declaration (id (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::Device"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Device")))))
    (declaration (id (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DeviceAlias")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Device")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::Device")))))
    (reference (id (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device"))) (kind featureTyping) (ordinal 0))
      (authored-target "DeviceAlias")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias")))))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias"))) (target (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::Device"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device"))) (target (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device"))) (target (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::Device"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::Device")))
      (subtype (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias")))
      (subtype (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device")))
      (type (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::Device")) (provenance implied))
      (type (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias")) (provenance authored))
      (effective-type (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::Device")) (source direct))
      (effective-type (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias")) (source direct))
      (supertype (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::Device")) (scopes any))
      (supertype (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/alias_target_binding.md") (range (start 2 26) (end 2 32)) (probe (position 2 26))
    (reference (id (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias"))) (kind aliasBinding) (ordinal 0) (authored-target "Device")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::Device")))))
    )
  )
  (query (document "memory://snapshot/alias_target_binding.md") (range (start 3 18) (end 3 29)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::device"))) (kind featureTyping) (ordinal 0) (authored-target "DeviceAlias")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_target_binding.md") (qualified-name "AliasCoverage::DeviceAlias")))))
    )
  )
)
~~~
