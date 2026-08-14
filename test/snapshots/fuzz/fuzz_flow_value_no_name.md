# META
~~~ini
description=Fuzz: flow usage with value and typing but no name preserves value in formatting
type=file
~~~
# SOURCE
~~~sysml
package P {
    part vehicle : Vehicle {
        part eng : Engine;

        flow = FuelFlow of Fuel
            from tank.fuelSupply
                to eng.engineFuelPort;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_flow_value_no_name.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 19) (end 1 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 19) (end 2 25))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "parser")
        (range (start 4 8) (end 7 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:6dc68fefb70a074b0c037f34f251cf4f9e3d4d84de07bfba423957f8759c84f8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_flow_value_no_name.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/fuzz_flow_value_no_name.md") (qualified-name "P::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/fuzz_flow_value_no_name.md") (qualified-name "P::vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/fuzz_flow_value_no_name.md") (qualified-name "P::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/fuzz_flow_value_no_name.md") (qualified-name "P::vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/fuzz_flow_value_no_name.md") (range (start 1 19) (end 1 26)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/fuzz_flow_value_no_name.md") (qualified-name "P::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/fuzz_flow_value_no_name.md") (range (start 2 19) (end 2 25)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/fuzz_flow_value_no_name.md") (qualified-name "P::vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
    )
  )
)
~~~
