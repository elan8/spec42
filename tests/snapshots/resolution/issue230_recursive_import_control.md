# META
~~~ini
description=Recursive imports span documents and resolve nested public members; explicit private visibility preserves the prior import semantics.
type=multi
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
## Library.sysml
~~~sysml
package Lib { package Nested { part def CelestialBody; } }
~~~
## Model.sysml
~~~sysml
package Model { private import Lib::**; part def Orbit { ref primary : CelestialBody; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind feature_typing) (source "Model::Orbit::primary")
    (target "Lib::Nested::CelestialBody") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/Library.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/Model.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:dc4ce6c9e9dbae85189fd420fb3b44ace41ab2a5935f9ab2ddae4ec7638a1bd7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib::Nested"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib::Nested::CelestialBody"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Lib") (import (shape membership) (recursive true))))))
    (declaration (id (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit::primary"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CelestialBody")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/Model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib")))))
    (reference (id (source (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit::primary"))) (kind featureTyping) (ordinal 0))
      (authored-target "CelestialBody")
      (outcome (status resolved) (target (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib::Nested::CelestialBody")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit::primary"))) (target (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib::Nested::CelestialBody"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit::primary"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit::primary"))) (target (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib::Nested::CelestialBody")))
      (subtype (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit::primary")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit::primary")))
      (featured-by (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit")))
      (type (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib::Nested::CelestialBody")) (provenance authored))
      (effective-type (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib::Nested::CelestialBody")) (source direct))
      (supertype (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib::Nested::CelestialBody")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/Model.sysml") (range (start 0 31) (end 0 38)) (probe (position 0 31))
    (reference (id (source (node (document "memory://snapshot/Model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib")))))
    )
  )
  (query (document "memory://snapshot/Model.sysml") (range (start 0 71) (end 0 84)) (probe (position 0 71))
    (reference (id (source (node (document "memory://snapshot/Model.sysml") (qualified-name "Model::Orbit::primary"))) (kind featureTyping) (ordinal 0) (authored-target "CelestialBody")
      (outcome (status resolved) (target (node (document "memory://snapshot/Library.sysml") (qualified-name "Lib::Nested::CelestialBody")))))
    )
  )
)
~~~
