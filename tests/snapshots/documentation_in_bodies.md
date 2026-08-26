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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:179775f22a464be7ba2dc441823cb6414b25e8568cf89ba186e3b8ffe59e2c5c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text " Package-level documentation. "))))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Car"))) (kind alias) (membership (kind alias) (visibility default)) (documentation (doc (text " Alias documentation. "))) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Color"))) (kind enum-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Enum def documentation. "))))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Color::red"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Payload"))) (kind item-def) (membership (kind owning) (visibility default)) (documentation (doc (locale "en") (text " Full form doc with short name and locale. "))))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Speed"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Named documentation. "))))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Part def documentation. "))))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle::speed"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (documentation (doc (text " Usage-level documentation. "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
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
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Color::red"))) (target (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Color"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle::speed"))) (target (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Color::red")))
      (featured-by (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Color")))
    )
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle")))
      (subtype (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle::speed")))
      (featured-by (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::vehicle")))
      (type (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle")) (scopes any))
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
  )
  (query (document "memory://snapshot/documentation_in_bodies.md") (range (start 17 19) (end 17 26)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/documentation_in_bodies.md") (qualified-name "DocTests::Vehicle")))))
    )
  )
)
~~~
