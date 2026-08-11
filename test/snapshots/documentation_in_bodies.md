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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPart,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,Ident,RegularComment,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwEnum,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwItem,KwDef,Ident,OpenCurly,
KwDoc,OpenAngle,Ident,CloseAngle,Ident,KwLocale,StringValue,RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'DocTests'
    (documentation)
    (part_def 'Vehicle'
      (documentation)
      (attribute_usage 'speed'))
    (attribute_def 'Speed'
      (documentation 'DocName'))
    (enum_def 'Color'
      (documentation)
      (enum_value 'red'))
    (part_usage 'vehicle' : 'Vehicle'
      (documentation))
    (item_def 'Payload'
      (documentation 'PayloadDoc' locale "en"))
    (alias_member 'Car' for 'Vehicle'
      (documentation))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f61ead110c2614ae1d5fa2bbd6b2da04825c7d88af73f87e82ed7889500726b9") (contract-version "canonical-resolution-v1"))
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
