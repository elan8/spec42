# META
~~~ini
description=A dotted subsetting target starts in the specializing feature's owning scope
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=Local name resolution for the target of a Feature-owned Specialization
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.2.3.5.3:resolveLocalName
type=file
libraries=none
~~~
# SOURCE
~~~kerml
package P {
    class Cell {
        feature that;
        feature vertices;
    }
    class Noise {
        feature that;
    }
    class Shape {
        feature vertices;
        feature cells : Cell {
            feature redefines that : Shape;
            feature selected : Noise subsets that.vertices;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/dotted_subsetting_owning_scope.md"
    (diagnostics
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind subsetting)
    (source "P::Shape::cells::selected")
    (target "P::Shape::vertices")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/dotted_subsetting_owning_scope.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b0cee264dfb37b7ec53b114be424be8838825dae4f645221870dfc7ab1ca839c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell::that"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell::vertices"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise::that"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cell")))))
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (effective-identification (name "that") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shape")) (redefinition (reference "that")))))
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Noise")) (subsetting (reference "that::vertices")))))
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::vertices"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cell")
      (outcome (status resolved) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell")))))
    (reference (id (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Shape")
      (outcome (status resolved) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape")))))
    (reference (id (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell::that")))))
    (reference (id (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "Noise")
      (outcome (status resolved) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise")))))
    (reference (id (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected"))) (kind subsetting) (ordinal 0))
      (authored-target "that::vertices")
      (outcome (status resolved) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::vertices")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells"))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell::that"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected"))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected"))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::vertices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell::that"))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell::vertices"))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise::that"))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells"))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected"))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::vertices"))) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell")))
      (subtype (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell::that")))
      (featured-by (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell")))
      (subtype (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell::vertices")))
      (featured-by (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell")))
    )
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise")))
      (subtype (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise::that")))
      (featured-by (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise")))
    )
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape")))
      (subtype (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells")))
      (featured-by (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape")))
      (type (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell")) (provenance authored))
      (effective-type (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell")) (source direct))
      (supertype (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells")))
      (type (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape")) (provenance authored))
      (effective-type (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape")) (source direct))
      (supertype (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell::that")) (scopes any feature))
      (supertype (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected")))
      (featured-by (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells")))
      (type (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise")) (provenance authored))
      (effective-type (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise")) (source direct))
      (supertype (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise")) (scopes any))
      (supertype (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::vertices")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::vertices")))
      (featured-by (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape")))
      (subtype (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/dotted_subsetting_owning_scope.md") (range (start 10 24) (end 10 28)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells"))) (kind featureTyping) (ordinal 0) (authored-target "Cell")
      (outcome (status resolved) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell")))))
    )
  )
  (query (document "memory://snapshot/dotted_subsetting_owning_scope.md") (range (start 11 37) (end 11 42)) (probe (position 11 37))
    (reference (id (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Shape")
      (outcome (status resolved) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape")))))
    )
  )
  (query (document "memory://snapshot/dotted_subsetting_owning_scope.md") (range (start 11 30) (end 11 34)) (probe (position 11 30))
    (reference (id (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (path (named (kind package) (name "P")) (named (kind class-def) (name "Shape")) (named (kind kerml-feature) (name "cells")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Cell::that")))))
    )
  )
  (query (document "memory://snapshot/dotted_subsetting_owning_scope.md") (range (start 12 31) (end 12 36)) (probe (position 12 31))
    (reference (id (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected"))) (kind featureTyping) (ordinal 0) (authored-target "Noise")
      (outcome (status resolved) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Noise")))))
    )
  )
  (query (document "memory://snapshot/dotted_subsetting_owning_scope.md") (range (start 12 45) (end 12 58)) (probe (position 12 45))
    (reference (id (source (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::cells::selected"))) (kind subsetting) (ordinal 0) (authored-target "that::vertices")
      (outcome (status resolved) (target (node (document "memory://snapshot/dotted_subsetting_owning_scope.md") (qualified-name "P::Shape::vertices")))))
    )
  )
)
~~~
