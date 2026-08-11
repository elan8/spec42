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
  (document "alias_target_binding.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AliasCoverage {
    part def Device;
    alias DeviceAlias for Device;
    part device : DeviceAlias;
}

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2f8ea047760deb543362ed302bcd674b0bc2387ec761b09f33c9eb7f2d2811e7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AliasCoverage"))) (kind "package") (name "AliasCoverage") (declared-name "AliasCoverage"))
    (element (id (node (document "d0") (qualified-name "AliasCoverage::Device"))) (kind "part def") (name "Device") (declared-name "Device") (parent (node (document "d0") (qualified-name "AliasCoverage"))))
    (element (id (node (document "d0") (qualified-name "AliasCoverage::DeviceAlias"))) (kind "alias") (name "DeviceAlias") (declared-name "DeviceAlias") (parent (node (document "d0") (qualified-name "AliasCoverage"))))
    (element (id (node (document "d0") (qualified-name "AliasCoverage::device"))) (kind "part") (name "device") (declared-name "device") (parent (node (document "d0") (qualified-name "AliasCoverage"))) (authored (membership (kind Feature)) (relationships (typing (reference "DeviceAlias")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AliasCoverage::device"))) (kind featureTyping) (ordinal 0)) (authored-target "DeviceAlias") (outcome (status resolved) (target (node (document "d0") (qualified-name "AliasCoverage::DeviceAlias")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AliasCoverage::device"))) (target (node (document "d0") (qualified-name "AliasCoverage::DeviceAlias"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AliasCoverage::device"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 18) (end 3 29)) (probe (position 3 18))
      (reference
        (source (document "d0") (qualified-name "AliasCoverage::device"))
        (kind featureTyping) (ordinal 0) (authored-target "DeviceAlias")
        (range (start 3 18) (end 3 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AliasCoverage::DeviceAlias") (range (start 2 4) (end 2 33)))
        )
      )
    )
  )
)
~~~
