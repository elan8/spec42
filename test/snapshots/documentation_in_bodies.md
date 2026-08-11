# META
~~~ini
description=Documentation comments in definition and usage bodies
type=file
~~~
# SOURCE
~~~sysml
package DocTests {
    doc /* Package-level documentation. */

    part def Vehicle {
        doc /* Part def documentation. */
        attribute speed;
    }

    attribute def Speed {
        doc DocName /* Named documentation. */
    }

    enum def Color {
        doc /* Enum def documentation. */
        enum red;
    }

    part vehicle : Vehicle {
        doc /* Usage-level documentation. */
    }

    item def Payload {
        doc <shortName> PayloadDoc locale "en" /* Full form doc with short name and locale. */
    }

    alias Car for Vehicle {
        doc /* Alias documentation. */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "documentation_in_bodies.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1cf8a24e6dc1e2baad52bb8841167eadf3426c2a162b48bdfa216435f910d90e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DocTests"))) (kind "package") (name "DocTests") (declared-name "DocTests"))
    (element (id (node (document "d0") (qualified-name "DocTests::Car"))) (kind "alias") (name "Car") (declared-name "Car") (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Color"))) (kind "enum def") (name "Color") (declared-name "Color") (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Color::red"))) (kind "enumerated value") (name "red") (declared-name "red") (parent (node (document "d0") (qualified-name "DocTests::Color"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Payload"))) (kind "item def") (name "Payload") (declared-name "Payload") (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Payload::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DocTests::Payload"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Speed"))) (kind "attribute def") (name "Speed") (declared-name "Speed") (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Speed::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DocTests::Speed"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Vehicle::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DocTests::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Vehicle::speed"))) (kind "attribute") (name "speed") (declared-name "speed") (parent (node (document "d0") (qualified-name "DocTests::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "DocTests::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "DocTests"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "DocTests::vehicle::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "DocTests::vehicle"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DocTests::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "DocTests::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "DocTests::vehicle"))) (target (node (document "d0") (qualified-name "DocTests::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DocTests::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 17 19) (end 17 26)) (probe (position 17 19))
      (reference
        (source (document "d0") (qualified-name "DocTests::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 17 19) (end 17 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DocTests::Vehicle") (range (start 3 4) (end 3 95)))
        )
      )
    )
  )
)
~~~
