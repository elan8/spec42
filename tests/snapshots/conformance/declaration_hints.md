# META
~~~ini
description=Untyped part usages, crossed multiplicity bounds and self-redefinition
type=file
~~~
# SOURCE
~~~sysml
package Declarations {
    part def Wheel;
    part typed : Wheel;
    part untyped;
    part crossedBounds : Wheel[3..1];
    part conformingBounds : Wheel[1..3];
    part def Redefines {
        attribute value :>> value;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/declaration_hints.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 3 4) (end 3 17))
      )
      (diagnostic
        (severity warning)
        (code "invalid_multiplicity")
        (source "semantic")
        (range (start 4 30) (end 4 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 28) (end 7 33))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ce5fb2e6dfee74afb7086d017849a3525b281172047b8fcf72eb813e71698ea0") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Redefines"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Redefines::value"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "value")))))
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::conformingBounds"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 3))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::crossedBounds"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 3) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::typed"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::untyped"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Redefines::value"))) (kind redefinition) (ordinal 0))
      (authored-target "value")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::conformingBounds"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::crossedBounds"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::typed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::conformingBounds"))) (target (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::conformingBounds"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::crossedBounds"))) (target (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::crossedBounds"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::typed"))) (target (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::typed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Redefines::value"))) (target (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Redefines"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Redefines::value")))
      (featured-by (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Redefines")))
    )
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")))
      (subtype (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::conformingBounds")) (scopes any))
      (subtype (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::crossedBounds")) (scopes any))
      (subtype (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::typed")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::conformingBounds")))
      (type (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::crossedBounds")))
      (type (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::typed")))
      (type (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/declaration_hints.md") (range (start 7 28) (end 7 33)) (probe (position 7 28))
    (reference (id (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Redefines::value"))) (kind redefinition) (ordinal 0) (authored-target "value")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/declaration_hints.md") (range (start 5 28) (end 5 33)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::conformingBounds"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")))))
    )
  )
  (query (document "memory://snapshot/declaration_hints.md") (range (start 4 25) (end 4 30)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::crossedBounds"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")))))
    )
  )
  (query (document "memory://snapshot/declaration_hints.md") (range (start 2 17) (end 2 22)) (probe (position 2 17))
    (reference (id (source (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::typed"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/declaration_hints.md") (qualified-name "Declarations::Wheel")))))
    )
  )
)
~~~
