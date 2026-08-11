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
  (document "fuzz_flow_value_no_name.md"
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
        (source "sysml")
        (range (start 4 8) (end 4 108))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c7a97b9d18e0ee31e371a3fdb03948e6160f424e7052f92c690b270f16f6721b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 180))))
    (element (id (node (document "d0") (qualified-name "P::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 1) (character 4)) (end (line 1) (character 166))) (parent (node (document "d0") (qualified-name "P"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 1) (character 19)) (end (line 1) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "P::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 2) (character 8)) (end (line 2) (character 26))) (parent (node (document "d0") (qualified-name "P::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 2) (character 19)) (end (line 2) (character 25)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 1) (character 19)) (end (line 1) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "P::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 2) (character 19)) (end (line 2) (character 25))) (outcome (status unresolved)))
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
  (document "d0"
    (query (range (start 2 19) (end 2 25)) (probe (position 2 19))
      (reference
        (source (document "d0") (qualified-name "P::vehicle::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 2 19) (end 2 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 19) (end 1 26)) (probe (position 1 19))
      (reference
        (source (document "d0") (qualified-name "P::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 1 19) (end 1 26))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
