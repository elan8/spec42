# META
~~~ini
description=A redefinition never resolves to the feature that authored it
type=file
~~~
# SOURCE
~~~sysml
package Redefines {
    part def Base {
        attribute status;
    }

    // The redefining feature carries the redefined feature's own name, so its owner's owned
    // members contain a same-named binding: itself. It must still reach the inherited one.
    part def Derived :> Base {
        attribute status;
    }

    // Nothing named `missing` is inherited, and the only candidate in scope is the redefining
    // feature itself. The published outcome is unresolved, not a feature that redefines itself.
    part def Orphan {
        attribute missing :>> missing;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/redefinition_excludes_itself.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 30) (end 14 37))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:000f626dd73411f5adeeb2134e719b6deab5291397433a10cbe72cfc3d959510") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base::status"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived::status"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Orphan"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Orphan::missing"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "missing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base")))))
    (reference (id (source (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Orphan::missing"))) (kind redefinition) (ordinal 0))
      (authored-target "missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived"))) (target (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base::status"))) (target (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived::status"))) (target (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base::status"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived::status"))) (target (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Orphan::missing"))) (target (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Orphan"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base")))
      (subtype (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base::status")))
      (featured-by (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base")))
      (subtype (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived::status")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived")))
      (supertype (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived::status")))
      (featured-by (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived")))
      (supertype (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base::status")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Orphan::missing")))
      (featured-by (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Orphan")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/redefinition_excludes_itself.md") (range (start 7 24) (end 7 28)) (probe (position 7 24))
    (reference (id (source (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Derived"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Base")))))
    )
  )
  (query (document "memory://snapshot/redefinition_excludes_itself.md") (range (start 14 30) (end 14 37)) (probe (position 14 30))
    (reference (id (source (node (document "memory://snapshot/redefinition_excludes_itself.md") (qualified-name "Redefines::Orphan::missing"))) (kind redefinition) (ordinal 0) (authored-target "missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
