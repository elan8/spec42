# META
~~~ini
description=Bare succession usages lower and resolve their first and then ends in occurrence and item definition bodies
type=file
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package Demo {
    occurrence def OccurrenceSequence {
        event occurrence firstEvent;
        event occurrence secondEvent;
        first firstEvent then secondEvent;
    }

    item def ItemSequence {
        event occurrence firstEvent;
        event occurrence secondEvent;
        first firstEvent then secondEvent;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/bare_succession_definition_bodies.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/bare_succession_definition_bodies.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2b1682eaeebe6814be34bff45e0840a557d63b907c7ec4f1f2c436bab31d0dda"))
  (declarations
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "firstEvent")) (succession (reference "secondEvent")))))
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::firstEvent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::secondEvent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "firstEvent")) (succession (reference "secondEvent")))))
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::firstEvent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::secondEvent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "firstEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::firstEvent")))))
    (reference (id (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "secondEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::secondEvent")))))
    (reference (id (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "firstEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::firstEvent")))))
    (reference (id (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "secondEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::secondEvent")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::firstEvent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::secondEvent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::firstEvent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::secondEvent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::firstEvent"))) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::secondEvent"))) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::firstEvent"))) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::secondEvent"))) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence")))
    )
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::firstEvent")))
      (featured-by (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence")))
    )
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::secondEvent")))
      (featured-by (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence")))
    )
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence")))
    )
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::firstEvent")))
      (featured-by (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence")))
    )
    (declaration (id (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::secondEvent")))
      (featured-by (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/bare_succession_definition_bodies.md") (range (start 10 14) (end 10 24)) (probe (position 10 14))
    (reference (id (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "firstEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::firstEvent")))))
    )
  )
  (query (document "memory://snapshot/bare_succession_definition_bodies.md") (range (start 10 30) (end 10 41)) (probe (position 10 30))
    (reference (id (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "ItemSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "secondEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::ItemSequence::secondEvent")))))
    )
  )
  (query (document "memory://snapshot/bare_succession_definition_bodies.md") (range (start 4 14) (end 4 24)) (probe (position 4 14))
    (reference (id (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "firstEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::firstEvent")))))
    )
  )
  (query (document "memory://snapshot/bare_succession_definition_bodies.md") (range (start 4 30) (end 4 41)) (probe (position 4 30))
    (reference (id (source (node (document "memory://snapshot/bare_succession_definition_bodies.md") (path (named (kind package) (name "Demo")) (named (kind occurrence-def) (name "OccurrenceSequence")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "secondEvent")
      (outcome (status resolved) (target (node (document "memory://snapshot/bare_succession_definition_bodies.md") (qualified-name "Demo::OccurrenceSequence::secondEvent")))))
    )
  )
)
~~~
