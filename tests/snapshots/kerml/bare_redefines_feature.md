# META
~~~ini
description=Bare redefines/subsets as shorthand features in KerML bodies
type=kerml
~~~
# SOURCE
~~~kerml
class A {
	feature innerSpaceDimension : Natural [1];
	portion feature all portions: Occurrence[1..*] {
		portion redefines portionOfLife = (that as Occurrence).portionOfLife;
	}
	feature all spaceTimeEnclosedPoints : Occurrence[1..*] {
		redefines innerSpaceDimension = 0;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/bare_redefines_feature.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 31) (end 1 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 31) (end 2 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 20) (end 3 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 36) (end 3 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 39) (end 5 49))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:bf8a3f17c1c71a61dbf0120f7205c5c1540c834b5e62fe275a3f2ad5e2d3ff1d") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::innerSpaceDimension"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Natural")))))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::portions"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers all portion) (multiplicity (lower 1) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")))))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (feature-value (kind bind) (value (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "portionOfLife")))))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "that::portionOfLife")))))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (kind kerml-feature) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::spaceTimeEnclosedPoints"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers all) (multiplicity (lower 1) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")))))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "innerSpaceDimension")))))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::innerSpaceDimension"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::portions"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "portionOfLife")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "that::portionOfLife")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::spaceTimeEnclosedPoints"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "innerSpaceDimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::innerSpaceDimension")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::innerSpaceDimension"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::innerSpaceDimension"))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::portions"))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::portions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::spaceTimeEnclosedPoints"))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::spaceTimeEnclosedPoints"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::innerSpaceDimension")))
      (featured-by (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A")))
      (subtype (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::portions")))
      (featured-by (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A")))
    )
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::portions")))
    )
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))))
      (subtype (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::spaceTimeEnclosedPoints")))
      (featured-by (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A")))
    )
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::spaceTimeEnclosedPoints")))
      (supertype (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::innerSpaceDimension")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/bare_redefines_feature.md") (range (start 1 31) (end 1 38)) (probe (position 1 31))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::innerSpaceDimension"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/bare_redefines_feature.md") (range (start 2 31) (end 2 41)) (probe (position 2 31))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::portions"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/bare_redefines_feature.md") (range (start 3 20) (end 3 33)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "portionOfLife")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/bare_redefines_feature.md") (range (start 3 36) (end 3 70)) (probe (position 3 36))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "portions")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "that::portionOfLife")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/bare_redefines_feature.md") (range (start 5 39) (end 5 49)) (probe (position 5 39))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::spaceTimeEnclosedPoints"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/bare_redefines_feature.md") (range (start 6 12) (end 6 31)) (probe (position 6 12))
    (reference (id (source (node (document "memory://snapshot/bare_redefines_feature.md") (path (named (kind class-def) (name "A")) (named (kind kerml-feature) (name "spaceTimeEnclosedPoints")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "innerSpaceDimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/bare_redefines_feature.md") (qualified-name "A::innerSpaceDimension")))))
    )
  )
)
~~~
