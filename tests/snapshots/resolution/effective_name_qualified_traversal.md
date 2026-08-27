# META
~~~ini
description=Qualified relationship paths traverse unnamed redefining Features by effective name
type=file
observed_gap=The relationship fixed point selected intermediate scopes by authored names only, excluding an unnamed Feature whose effective name comes from its first Redefinition.
~~~
# SOURCE
~~~sysml
package Demo {
    item def Cell {
        item edge;
        ref item probe :>> Mid::faces::edge;
    }

    item def Base {
        item faces : Cell;
    }

    item def Mid :> Base {
        item :>> faces;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind redefinition)
    (source "Demo::Cell::probe")
    (target "Demo::Cell::edge")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/effective_name_qualified_traversal.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e54b7472d8b08f2e9d36b212de49cd02da81e96fde6be91caf33884e2ce51734"))
  (declarations
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cell")))))
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::edge"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::probe"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Mid::faces::edge")))))
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Mid"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "Mid")) (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (effective-identification (name "faces") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "faces")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cell")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell")))))
    (reference (id (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::probe"))) (kind redefinition) (ordinal 0))
      (authored-target "Mid::faces::edge")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::edge")))))
    (reference (id (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Mid"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base")))))
    (reference (id (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "Mid")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "faces")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces"))) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::probe"))) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::edge"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::probe"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Mid"))) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Mid"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "Mid")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "Mid")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces"))) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::edge"))) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::probe"))) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "Mid")) (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Mid"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base")))
      (subtype (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Mid")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces")))
      (featured-by (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base")))
      (type (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell")) (provenance authored))
      (effective-type (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell")) (source direct))
      (supertype (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell")) (scopes any))
      (subtype (node (document "memory://snapshot/effective_name_qualified_traversal.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "Mid")) (anonymous (kind item) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell")))
      (subtype (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::edge")))
      (featured-by (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell")))
      (subtype (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::probe")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::probe")))
      (featured-by (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell")))
      (supertype (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::edge")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Mid")))
      (supertype (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/effective_name_qualified_traversal.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "Mid")) (anonymous (kind item) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Mid")))
      (effective-type (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell")) (source inherited) (from (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces"))))
      (supertype (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces")) (scopes any feature))
      (supertype (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/effective_name_qualified_traversal.md") (range (start 7 21) (end 7 25)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces"))) (kind featureTyping) (ordinal 0) (authored-target "Cell")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell")))))
    )
  )
  (query (document "memory://snapshot/effective_name_qualified_traversal.md") (range (start 3 27) (end 3 43)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::probe"))) (kind redefinition) (ordinal 0) (authored-target "Mid::faces::edge")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Cell::edge")))))
    )
  )
  (query (document "memory://snapshot/effective_name_qualified_traversal.md") (range (start 10 20) (end 10 24)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Mid"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base")))))
    )
  )
  (query (document "memory://snapshot/effective_name_qualified_traversal.md") (range (start 11 17) (end 11 22)) (probe (position 11 17))
    (reference (id (source (node (document "memory://snapshot/effective_name_qualified_traversal.md") (path (named (kind package) (name "Demo")) (named (kind item-def) (name "Mid")) (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "faces")
      (outcome (status resolved) (target (node (document "memory://snapshot/effective_name_qualified_traversal.md") (qualified-name "Demo::Base::faces")))))
    )
  )
)
~~~
