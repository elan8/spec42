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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "DocTests"))) (name "DocTests") (declared-name "DocTests")
      (contains
        (element (kind "alias") (id (node (document "d0") (qualified-name "DocTests::Car"))) (name "Car") (declared-name "Car"))
        (element (kind "enum def") (id (node (document "d0") (qualified-name "DocTests::Color"))) (name "Color") (declared-name "Color")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "DocTests::Color::red"))) (name "red") (declared-name "red") (effective (featuring-type (node (document "d0") (qualified-name "DocTests::Color")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "DocTests::Payload"))) (name "Payload") (declared-name "Payload")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "DocTests::Payload::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "DocTests::Payload")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "DocTests::Speed"))) (name "Speed") (declared-name "Speed") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "DocTests::Speed::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "DocTests::Speed")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "DocTests::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "DocTests::Vehicle::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "DocTests::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "DocTests::Vehicle::speed"))) (name "speed") (declared-name "speed") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "DocTests::Vehicle")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "DocTests::_documentation"))) (name ""))
        (element (kind "part") (id (node (document "d0") (qualified-name "DocTests::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "DocTests::vehicle::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "DocTests::Vehicle")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DocTests::Payload::_documentation"))) (to (node (document "d0") (qualified-name "DocTests::Payload"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DocTests::Speed::_documentation"))) (to (node (document "d0") (qualified-name "DocTests::Speed"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DocTests::Vehicle::_documentation"))) (to (node (document "d0") (qualified-name "DocTests::Vehicle"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DocTests::_documentation"))) (to (node (document "d0") (qualified-name "DocTests"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DocTests::vehicle::_documentation"))) (to (node (document "d0") (qualified-name "DocTests::vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "DocTests::vehicle"))) (to (node (document "d0") (qualified-name "DocTests::Vehicle"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
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
