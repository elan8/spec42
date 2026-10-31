# META
~~~ini
description=Canonical import planning: import combined collisions
type=file
libraries=none
~~~
# SOURCE
## model.sysml
~~~sysml
package PartsCatalog { part def Same; part def Good; }
package PortsCatalog { port def Same; }
package Machine {
    part p : Same;
    port q : Same;
    part safe : Good;
}
~~~
# EDITOR QUERIES
~~~text
probe model.sysml 3 13 imports=Sa fix-imports
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
        (range (start 3 13) (end 3 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 13) (end 4 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 16) (end 5 20))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:dafc48e0d32e566e76efb8c1c7690d4a41b0d837a5006014c679490d51b4bad0"))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Same")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::q"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Same")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::safe"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Good")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "PartsCatalog"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "PartsCatalog::Good"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "PartsCatalog::Same"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "PortsCatalog"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "PortsCatalog::Same"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "PortsCatalog")) (named (kind port-def) (name "Same")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Same") (short-name absent) (provenance original-port-definition)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Same")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Same")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::safe"))) (kind featureTyping) (ordinal 0))
      (authored-target "Good")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "PortsCatalog")) (named (kind port-def) (name "Same")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "PortsCatalog::Same"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 3 13) (end 3 17)) (probe (position 3 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::p"))) (kind featureTyping) (ordinal 0) (authored-target "Same")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 13) (end 4 17)) (probe (position 4 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::q"))) (kind featureTyping) (ordinal 0) (authored-target "Same")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 16) (end 5 20)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::safe"))) (kind featureTyping) (ordinal 0) (authored-target "Good")
      (outcome (status unresolved)))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/model.sysml") (position 3 13)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "Machine") (qualified-name "Machine") (kind "Package")) (member (name "PartsCatalog") (qualified-name "PartsCatalog") (kind "Package")) (member (name "PortsCatalog") (qualified-name "PortsCatalog") (kind "Package")) (member (name "p") (qualified-name "Machine::p") (kind "PartUsage")) (member (name "q") (qualified-name "Machine::q") (kind "PortUsage")) (member (name "safe") (qualified-name "Machine::safe") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "p")
          (qualified-name "Machine::p")
          (location (document "memory://snapshot/model.sysml") (range (start 3 9) (end 3 10)) (role Declaration))
          (declaration (range (start 3 4) (end 3 18)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Same") (target unresolved))
          (typing (outcome unresolved))
          (effective-typing (outcome unresolved))
        )
      )
      (reference-kind featureTyping)
      (referenced (status unresolved))
    )
    (completion-imports (status resolved))
    (missing-imports (status resolved) (import (target "PartsCatalog::Good") (referent "PartsCatalog::Good") (spelling "Good") (namespace "Machine") (range (start 5 16) (end 5 20))) (repaired (range (start 5 16) (end 5 20))) (remaining (Conflict (range (start 3 13) (end 3 17))) (Conflict (range (start 4 13) (end 4 17)))))
  )
  (document-symbols (document "memory://snapshot/model.sysml")
    (status resolved)
    (symbol (kind "Package") (name "PartsCatalog") (qualified-name "PartsCatalog") (location (document "memory://snapshot/model.sysml") (range (start 0 8) (end 0 20)) (role Declaration)) (declaration (range (start 0 0) (end 0 54))))
    (symbol (kind "PartDefinition") (name "Same") (qualified-name "PartsCatalog::Same") (location (document "memory://snapshot/model.sysml") (range (start 0 32) (end 0 36)) (role Declaration)) (declaration (range (start 0 23) (end 0 37))))
    (symbol (kind "PartDefinition") (name "Good") (qualified-name "PartsCatalog::Good") (location (document "memory://snapshot/model.sysml") (range (start 0 47) (end 0 51)) (role Declaration)) (declaration (range (start 0 38) (end 0 52))))
    (symbol (kind "Package") (name "PortsCatalog") (qualified-name "PortsCatalog") (location (document "memory://snapshot/model.sysml") (range (start 1 8) (end 1 20)) (role Declaration)) (declaration (range (start 1 0) (end 1 39))))
    (symbol (kind "PortDefinition") (name "Same") (qualified-name "PortsCatalog::Same") (location (document "memory://snapshot/model.sysml") (range (start 1 32) (end 1 36)) (role Declaration)) (declaration (range (start 1 23) (end 1 37))))
    (symbol (kind "Package") (name "Machine") (qualified-name "Machine") (location (document "memory://snapshot/model.sysml") (range (start 2 8) (end 2 15)) (role Declaration)) (declaration (range (start 2 0) (end 6 1))))
    (symbol (kind "PartUsage") (name "p") (qualified-name "Machine::p") (location (document "memory://snapshot/model.sysml") (range (start 3 9) (end 3 10)) (role Declaration)) (declaration (range (start 3 4) (end 3 18))))
    (symbol (kind "PortUsage") (name "q") (qualified-name "Machine::q") (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 10)) (role Declaration)) (declaration (range (start 4 4) (end 4 18))))
    (symbol (kind "PartUsage") (name "safe") (qualified-name "Machine::safe") (location (document "memory://snapshot/model.sysml") (range (start 5 9) (end 5 13)) (role Declaration)) (declaration (range (start 5 4) (end 5 21))))
  )
)
~~~
