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
  (document "memory://snapshot/documentation_in_bodies.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 21 4) (end 23 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:179775f22a464be7ba2dc441823cb6414b25e8568cf89ba186e3b8ffe59e2c5c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Car"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Color"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Color::red"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Speed"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle::speed"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Car"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle")))))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Car"))) (target (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Car"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::vehicle"))) (target (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/documentation_in_bodies.md") (range (start 25 18) (end 25 25)) (probe (position 25 18))
    (reference (id (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Car"))) (kind aliasBinding) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle")))))
  )
  (query (document "memory://snapshot/documentation_in_bodies.md") (range (start 17 19) (end 17 26)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle")))))
  )
)
~~~
