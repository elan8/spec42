# META
~~~ini
description=Canonical import planning: import multiple namespaces
type=file
libraries=none
~~~
# SOURCE
## model.sysml
~~~sysml
package Catalog { part def Motor; port def Plug; }
package One { part motor : Motor; } package Two { port plug : Plug; }
package Outer {
    private import Catalog::Motor;
    part existing : Motor;
    package Inner { port plug : Plug; }
}
~~~
# EDITOR QUERIES
~~~text
probe model.sysml 1 27 imports=Mot fix-imports
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 27) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 62) (end 1 66))
      )
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 3 19) (end 3 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 32) (end 5 36))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ed2340872a16f099d3fc053d7d2f0f30a1f4310c2c098481594e22ea28d93ec3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Catalog")) (named (kind port-def) (name "Plug")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Plug") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "One"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "One::motor"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Outer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Outer")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Catalog::Motor") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::Inner"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::Inner::plug"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Plug")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::existing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Two"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Two::plug"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Plug")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "One::motor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Outer")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Catalog::Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::Inner::plug"))) (kind featureTyping) (ordinal 0))
      (authored-target "Plug")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::existing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Two::plug"))) (kind featureTyping) (ordinal 0))
      (authored-target "Plug")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::existing"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::existing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Catalog")) (named (kind port-def) (name "Plug")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::existing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::existing")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 27) (end 1 32)) (probe (position 1 27))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "One::motor"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 19) (end 3 33)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Outer")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Catalog::Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 32) (end 5 36)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::Inner::plug"))) (kind featureTyping) (ordinal 0) (authored-target "Plug")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 20) (end 4 25)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Outer::existing"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 1 62) (end 1 66)) (probe (position 1 62))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Two::plug"))) (kind featureTyping) (ordinal 0) (authored-target "Plug")
      (outcome (status unresolved)))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/model.sysml") (position 1 27)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "Catalog") (qualified-name "Catalog") (kind "Package")) (member (name "One") (qualified-name "One") (kind "Package")) (member (name "Outer") (qualified-name "Outer") (kind "Package")) (member (name "Two") (qualified-name "Two") (kind "Package")) (member (name "motor") (qualified-name "One::motor") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "motor")
          (qualified-name "One::motor")
          (location (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 24)) (role Declaration))
          (declaration (range (start 1 14) (end 1 33)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Motor") (target unresolved))
          (typing (outcome unresolved))
          (effective-typing (outcome unresolved))
        )
      )
      (reference-kind featureTyping)
      (referenced (status unresolved))
    )
    (completion-imports (status resolved) (import (target "Catalog::Motor") (referent "Catalog::Motor") (spelling "Motor") (namespace "One") (range (start 1 27) (end 1 32))) (safe true))
    (missing-imports (status resolved) (import (target "Catalog::Motor") (referent "Catalog::Motor") (spelling "Motor") (namespace "One") (range (start 1 27) (end 1 32))) (import (target "Catalog::Plug") (referent "Catalog::Plug") (spelling "Plug") (namespace "Two") (range (start 1 62) (end 1 66))) (import (target "Catalog::Plug") (referent "Catalog::Plug") (spelling "Plug") (namespace "Outer::Inner") (range (start 5 32) (end 5 36))) (repaired (range (start 1 27) (end 1 32)) (range (start 1 62) (end 1 66)) (range (start 5 32) (end 5 36))) (remaining))
  )
  (document-symbols (document "memory://snapshot/model.sysml")
    (status resolved)
    (symbol (kind "Package") (name "Catalog") (qualified-name "Catalog") (location (document "memory://snapshot/model.sysml") (range (start 0 8) (end 0 15)) (role Declaration)) (declaration (range (start 0 0) (end 0 50))))
    (symbol (kind "PartDefinition") (name "Motor") (qualified-name "Catalog::Motor") (location (document "memory://snapshot/model.sysml") (range (start 0 27) (end 0 32)) (role Declaration)) (declaration (range (start 0 18) (end 0 33))))
    (symbol (kind "PortDefinition") (name "Plug") (qualified-name "Catalog::Plug") (location (document "memory://snapshot/model.sysml") (range (start 0 43) (end 0 47)) (role Declaration)) (declaration (range (start 0 34) (end 0 48))))
    (symbol (kind "Package") (name "One") (qualified-name "One") (location (document "memory://snapshot/model.sysml") (range (start 1 8) (end 1 11)) (role Declaration)) (declaration (range (start 1 0) (end 1 35))))
    (symbol (kind "PartUsage") (name "motor") (qualified-name "One::motor") (location (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 24)) (role Declaration)) (declaration (range (start 1 14) (end 1 33))))
    (symbol (kind "Package") (name "Two") (qualified-name "Two") (location (document "memory://snapshot/model.sysml") (range (start 1 44) (end 1 47)) (role Declaration)) (declaration (range (start 1 36) (end 1 69))))
    (symbol (kind "PortUsage") (name "plug") (qualified-name "Two::plug") (location (document "memory://snapshot/model.sysml") (range (start 1 55) (end 1 59)) (role Declaration)) (declaration (range (start 1 50) (end 1 67))))
    (symbol (kind "Package") (name "Outer") (qualified-name "Outer") (location (document "memory://snapshot/model.sysml") (range (start 2 8) (end 2 13)) (role Declaration)) (declaration (range (start 2 0) (end 6 1))))
    (symbol (kind "Import") (qualified-name "Outer::") (location (document "memory://snapshot/model.sysml") (range (start 3 4) (end 3 34)) (role Declaration)) (declaration (range (start 3 4) (end 3 34))))
    (symbol (kind "PartUsage") (name "existing") (qualified-name "Outer::existing") (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 17)) (role Declaration)) (declaration (range (start 4 4) (end 4 26))))
    (symbol (kind "Package") (name "Inner") (qualified-name "Outer::Inner") (location (document "memory://snapshot/model.sysml") (range (start 5 12) (end 5 17)) (role Declaration)) (declaration (range (start 5 4) (end 5 39))))
    (symbol (kind "PortUsage") (name "plug") (qualified-name "Outer::Inner::plug") (location (document "memory://snapshot/model.sysml") (range (start 5 25) (end 5 29)) (role Declaration)) (declaration (range (start 5 20) (end 5 37))))
  )
)
~~~
