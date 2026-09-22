# META
~~~ini
description=Port definition and usage parse with and without a body, including nested ports and conjugated typing
type=file
~~~
# SOURCE
~~~sysml
package Ports {
    port def Bare;
    port def Braced { }
    port def Power {
        attribute voltage;
        out item current;
        port nested : Bare;
    }
    part def Device {
        port p : Power;
        port q : ~Power;
        port empty;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/port_def_with_and_without_body.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 6 8) (end 6 27))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 9 8) (end 9 23))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 10 8) (end 10 24))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 11 8) (end 11 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:933f97f8c57003a7ecc79f7594e7a5e372cc228f5eac1378303baec0c29ee2c8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Bare"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Braced"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::empty"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::p"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Power")))))
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::q"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Power") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::current"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::nested"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bare")))))
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::voltage"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")))))
    (reference (id (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")))))
    (reference (id (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::nested"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bare")
      (outcome (status resolved) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Bare")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::p"))) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::q"))) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::nested"))) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Bare"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::nested"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::empty"))) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::p"))) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::q"))) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::current"))) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::nested"))) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::voltage"))) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Bare")))
      (subtype (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::nested")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::empty")))
      (featured-by (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device")))
    )
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::p")))
      (featured-by (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device")))
      (type (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")) (provenance authored))
      (effective-type (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")) (source direct))
      (supertype (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::q")))
      (featured-by (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device")))
      (type (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")) (provenance authored))
      (effective-type (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")) (source direct))
      (supertype (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")))
      (subtype (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::p")) (scopes any))
      (subtype (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::q")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::current")))
      (featured-by (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")))
    )
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::nested")))
      (featured-by (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")))
      (type (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Bare")) (provenance authored))
      (effective-type (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Bare")) (source direct))
      (supertype (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Bare")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::voltage")))
      (featured-by (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/port_def_with_and_without_body.md") (range (start 9 17) (end 9 22)) (probe (position 9 17))
    (reference (id (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::p"))) (kind featureTyping) (ordinal 0) (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")))))
    )
  )
  (query (document "memory://snapshot/port_def_with_and_without_body.md") (range (start 10 18) (end 10 23)) (probe (position 10 18))
    (reference (id (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Device::q"))) (kind featureTyping) (ordinal 0) (authored-target "Power")
      (outcome (status resolved) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power")))))
    )
  )
  (query (document "memory://snapshot/port_def_with_and_without_body.md") (range (start 6 22) (end 6 26)) (probe (position 6 22))
    (reference (id (source (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Power::nested"))) (kind featureTyping) (ordinal 0) (authored-target "Bare")
      (outcome (status resolved) (target (node (document "memory://snapshot/port_def_with_and_without_body.md") (qualified-name "Ports::Bare")))))
    )
  )
)
~~~
