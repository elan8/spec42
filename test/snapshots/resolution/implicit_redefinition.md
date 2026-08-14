# META
~~~ini
description=Model diagnostics for implicit inherited feature redefinition
type=file
~~~
# SOURCE
~~~sysml
package P {
    part def Base {
        attribute mass : Real;
    }
    part def Child :> Base {
        attribute mass = 1200;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/implicit_redefinition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 25) (end 2 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:74d9089eda8be44c1433e4fddd86bfcbe49de9351c5133dee8eaaeae0eb62939") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Base::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Child"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base"))))
    (declaration (id (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Child::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Base::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Base")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Child"))) (target (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Child::mass"))) (target (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Base::mass"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Child::mass"))) (state literal) (value (kind integer) (integer 1200)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/implicit_redefinition.md") (range (start 2 25) (end 2 29)) (probe (position 2 25))
    (reference (id (source (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Base::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/implicit_redefinition.md") (range (start 4 22) (end 4 26)) (probe (position 4 22))
    (reference (id (source (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Child"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/implicit_redefinition.md") (qualified-name "P::Base")))))
  )
)
~~~
