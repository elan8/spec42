# META
~~~ini
description=Canonical import planning: import path shadowing
type=file
libraries=none
~~~
# SOURCE
## model.sysml
~~~sysml
package A { part def B; part def Good; }
package B { port def Plug; }
package Machine {
    port existing : B::Plug;
    part unsafe : B;
    part safe : Good;
}
~~~
# EDITOR QUERIES
~~~text
probe model.sysml 4 18 imports=B fix-imports
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
        (range (start 5 16) (end 5 20))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3be3dd882aca5a71a59ee056bdc76613bc11bf15064266dec59dbad3e91c5820"))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "A"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "A::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "A::Good"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "B"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "B::Plug"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "B")) (named (kind port-def) (name "Plug")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Plug") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::existing"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B::Plug")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::safe"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Good")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::unsafe"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::existing"))) (kind featureTyping) (ordinal 0))
      (authored-target "B::Plug")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "B::Plug")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::safe"))) (kind featureTyping) (ordinal 0))
      (authored-target "Good")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::unsafe"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::existing"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "B::Plug"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::existing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::unsafe"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::unsafe"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "B")) (named (kind port-def) (name "Plug")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "B::Plug"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "B")))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::unsafe")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "B::Plug")))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::existing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::existing")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "B::Plug")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "B::Plug")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "B::Plug")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::unsafe")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "B")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "B")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 3 20) (end 3 27)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::existing"))) (kind featureTyping) (ordinal 0) (authored-target "B::Plug")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "B::Plug")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 16) (end 5 20)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::safe"))) (kind featureTyping) (ordinal 0) (authored-target "Good")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 18) (end 4 19)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::unsafe"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "B")))))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/model.sysml") (position 4 18)
    (target (status resolved) (candidate (name "B") (location (document "memory://snapshot/model.sysml") (range (start 1 8) (end 1 9)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/model.sysml") (range (start 1 8) (end 1 9)) (role Declaration)) (location (document "memory://snapshot/model.sysml") (range (start 4 18) (end 4 19)) (role Reference))))
    (rename (status ready) (name "B") (range (start 4 18) (end 4 19)) (occurrences 2))
    (visible-members (candidates (member (name "A") (qualified-name "A") (kind "Package")) (member (name "B") (qualified-name "B") (kind "Package")) (member (name "Machine") (qualified-name "Machine") (kind "Package")) (member (name "Plug") (qualified-name "B::Plug") (kind "PortDefinition")) (member (name "existing") (qualified-name "Machine::existing") (kind "PortUsage")) (member (name "safe") (qualified-name "Machine::safe") (kind "PartUsage")) (member (name "unsafe") (qualified-name "Machine::unsafe") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "unsafe")
          (qualified-name "Machine::unsafe")
          (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 15)) (role Declaration))
          (declaration (range (start 4 4) (end 4 20)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "B") (target resolved))
          (typing (outcome resolved) (target "B"))
          (effective-typing (outcome resolved) (type (qualified-name "B") (origin direct) (provenance authored)))
          (outgoing (kind "typing") (peer "B") (provenance authored))
        )
      )
      (reference-kind featureTyping)
      (referenced (status resolved)
        (element (kind "Package")
          (name "B")
          (qualified-name "B")
          (location (document "memory://snapshot/model.sysml") (range (start 1 8) (end 1 9)) (role Declaration))
          (declaration (range (start 1 0) (end 1 28)))
          (membership (kind owning) (visibility public) (provenance default))
          (incoming (kind "typing") (peer "Machine::unsafe") (provenance authored))
        )
      )
    )
    (completion-imports (status resolved))
    (missing-imports (status resolved) (import (target "A::Good") (referent "A::Good") (spelling "Good") (namespace "Machine") (range (start 5 16) (end 5 20))) (repaired (range (start 5 16) (end 5 20))) (remaining))
  )
  (document-symbols (document "memory://snapshot/model.sysml")
    (status resolved)
    (symbol (kind "Package") (name "A") (qualified-name "A") (location (document "memory://snapshot/model.sysml") (range (start 0 8) (end 0 9)) (role Declaration)) (declaration (range (start 0 0) (end 0 40))))
    (symbol (kind "PartDefinition") (name "B") (qualified-name "A::B") (location (document "memory://snapshot/model.sysml") (range (start 0 21) (end 0 22)) (role Declaration)) (declaration (range (start 0 12) (end 0 23))))
    (symbol (kind "PartDefinition") (name "Good") (qualified-name "A::Good") (location (document "memory://snapshot/model.sysml") (range (start 0 33) (end 0 37)) (role Declaration)) (declaration (range (start 0 24) (end 0 38))))
    (symbol (kind "Package") (name "B") (qualified-name "B") (location (document "memory://snapshot/model.sysml") (range (start 1 8) (end 1 9)) (role Declaration)) (declaration (range (start 1 0) (end 1 28))))
    (symbol (kind "PortDefinition") (name "Plug") (qualified-name "B::Plug") (location (document "memory://snapshot/model.sysml") (range (start 1 21) (end 1 25)) (role Declaration)) (declaration (range (start 1 12) (end 1 26))))
    (symbol (kind "Package") (name "Machine") (qualified-name "Machine") (location (document "memory://snapshot/model.sysml") (range (start 2 8) (end 2 15)) (role Declaration)) (declaration (range (start 2 0) (end 6 1))))
    (symbol (kind "PortUsage") (name "existing") (qualified-name "Machine::existing") (location (document "memory://snapshot/model.sysml") (range (start 3 9) (end 3 17)) (role Declaration)) (declaration (range (start 3 4) (end 3 28))))
    (symbol (kind "PartUsage") (name "unsafe") (qualified-name "Machine::unsafe") (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 15)) (role Declaration)) (declaration (range (start 4 4) (end 4 20))))
    (symbol (kind "PartUsage") (name "safe") (qualified-name "Machine::safe") (location (document "memory://snapshot/model.sysml") (range (start 5 9) (end 5 13)) (role Declaration)) (declaration (range (start 5 4) (end 5 21))))
  )
)
~~~
