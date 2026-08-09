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
(model
  (namespace
    (package 'DocTests'
      (documentation)
      (part_def 'Vehicle'
        (documentation)
        (attribute_usage composite 'speed'))
      (attribute_def 'Speed'
        (documentation 'DocName'))
      (enum_def 'Color'
        (documentation)
        (enum_usage composite 'red'))
      (part_usage 'vehicle' : 'DocTests::Vehicle'[part_def]
        (documentation))
      (item_def 'Payload'
        (documentation 'PayloadDoc'))
      (alias_member 'Car' -> 'DocTests::Vehicle'[part_def]))))
~~~
