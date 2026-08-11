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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1cf8a24e6dc1e2baad52bb8841167eadf3426c2a162b48bdfa216435f910d90e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DocTests"))) (kind "package") (name "DocTests") (declared-name "DocTests") (range (start (line 0) (character 0)) (end (line 0) (character 608))))
    (element (id (node (document "d0") (qualified-name "DocTests::Car"))) (kind "alias") (name "Car") (declared-name "Car") (range (start (line 25) (character 4)) (end (line 25) (character 72))) (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Color"))) (kind "enum def") (name "Color") (declared-name "Color") (range (start (line 12) (character 4)) (end (line 12) (character 86))) (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Color::red"))) (kind "enumerated value") (name "red") (declared-name "red") (range (start (line 14) (character 13)) (end (line 14) (character 16))) (parent (node (document "d0") (qualified-name "DocTests::Color"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Payload"))) (kind "item def") (name "Payload") (declared-name "Payload") (range (start (line 21) (character 4)) (end (line 21) (character 123))) (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Payload::_documentation"))) (kind "documentation") (name "") (range (start (line 21) (character 4)) (end (line 21) (character 123))) (parent (node (document "d0") (qualified-name "DocTests::Payload"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Speed"))) (kind "attribute def") (name "Speed") (declared-name "Speed") (range (start (line 8) (character 4)) (end (line 8) (character 78))) (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Speed::_documentation"))) (kind "documentation") (name "") (range (start (line 8) (character 4)) (end (line 8) (character 78))) (parent (node (document "d0") (qualified-name "DocTests::Speed"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 3) (character 4)) (end (line 3) (character 95))) (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Vehicle::_documentation"))) (kind "documentation") (name "") (range (start (line 3) (character 4)) (end (line 3) (character 95))) (parent (node (document "d0") (qualified-name "DocTests::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "DocTests::Vehicle::speed"))) (kind "attribute") (name "speed") (declared-name "speed") (range (start (line 5) (character 8)) (end (line 5) (character 24))) (parent (node (document "d0") (qualified-name "DocTests::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "DocTests::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 608))) (parent (node (document "d0") (qualified-name "DocTests"))))
    (element (id (node (document "d0") (qualified-name "DocTests::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 17) (character 4)) (end (line 17) (character 79))) (parent (node (document "d0") (qualified-name "DocTests"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 17) (character 19)) (end (line 17) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "DocTests::vehicle::_documentation"))) (kind "documentation") (name "") (range (start (line 17) (character 4)) (end (line 17) (character 79))) (parent (node (document "d0") (qualified-name "DocTests::vehicle"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DocTests::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 17) (character 19)) (end (line 17) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "DocTests::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "DocTests::vehicle"))) (target (node (document "d0") (qualified-name "DocTests::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "DocTests::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
