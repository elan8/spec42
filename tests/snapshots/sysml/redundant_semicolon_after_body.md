# META
~~~ini
description=A braced body already terminates its definition or usage, so a ';' written directly after the closing '}' terminates nothing and is reported as an error; the same members without the ';' are accepted
type=file
~~~
# SOURCE
~~~sysml
package Stray {
    part def Engine {
        attribute mass;
    };
    part def Vehicle {
        part engine : Engine {
        };
    }
    part def Clean {
        part engine : Engine { }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/redundant_semicolon_after_body.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "redundant_semicolon_after_body")
        (source "parser")
        (range (start 3 5) (end 3 6))
      )
      (diagnostic
        (severity error)
        (code "redundant_semicolon_after_body")
        (source "parser")
        (range (start 6 9) (end 6 10))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/redundant_semicolon_after_body.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "redundant_semicolon_after_body")
        (source "parser")
        (range (start 3 5) (end 3 6))
      )
      (diagnostic
        (severity error)
        (code "redundant_semicolon_after_body")
        (source "parser")
        (range (start 6 9) (end 6 10))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:7b4fe50e6e0891b7d9e615e6f03b358882e026713e43ef6fbb3a38ab2af077cd"))
  (declarations
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (path (named (kind package) (name "Stray")) (anonymous (kind default-reference) (ordinal 0))))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine::mass"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (path (named (kind package) (name "Stray")) (named (kind part-def) (name "Vehicle")) (anonymous (kind default-reference) (ordinal 0))))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")))))
    (reference (id (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean::engine"))) (target (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle::engine"))) (target (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean::engine"))) (target (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine::mass"))) (target (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (path (named (kind package) (name "Stray")) (named (kind part-def) (name "Vehicle")) (anonymous (kind default-reference) (ordinal 0))))) (target (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle::engine"))) (target (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean::engine")))
      (featured-by (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean")))
      (type (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")))
      (subtype (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean::engine")) (scopes any))
      (subtype (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle::engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine::mass")))
      (featured-by (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (path (named (kind package) (name "Stray")) (named (kind part-def) (name "Vehicle")) (anonymous (kind default-reference) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle::engine")))
      (featured-by (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle")))
      (type (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/redundant_semicolon_after_body.md") (range (start 9 22) (end 9 28)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Clean::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")))))
    )
  )
  (query (document "memory://snapshot/redundant_semicolon_after_body.md") (range (start 5 22) (end 5 28)) (probe (position 5 22))
    (reference (id (source (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/redundant_semicolon_after_body.md") (qualified-name "Stray::Engine")))))
    )
  )
)
~~~
