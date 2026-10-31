# META
~~~ini
description=Canonical import planning: import duplicate candidates
type=file
libraries=none
~~~
# SOURCE
## model.sysml
~~~sysml
package A { part def Motor; }
package B { part def Motor; }
package Machine {
    part engine : Mot;
    part other : Motor;
}
~~~
# EDITOR QUERIES
~~~text
probe model.sysml 3 18 imports=Mot fix-imports
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
        (range (start 3 18) (end 3 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 17) (end 4 22))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:95553710c6f656bd0588343a474bce61bca19fa0bdd13e5a85c22692006367da"))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "A"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "A::Motor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "B"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "B::Motor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Mot")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::other"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Mot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::other"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status unresolved)))
  )
  (relationships
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
  (query (document "memory://snapshot/model.sysml") (range (start 3 18) (end 3 21)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Mot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 17) (end 4 22)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::other"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status unresolved)))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/model.sysml") (position 3 18)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "A") (qualified-name "A") (kind "Package")) (member (name "B") (qualified-name "B") (kind "Package")) (member (name "Machine") (qualified-name "Machine") (kind "Package")) (member (name "engine") (qualified-name "Machine::engine") (kind "PartUsage")) (member (name "other") (qualified-name "Machine::other") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "engine")
          (qualified-name "Machine::engine")
          (location (document "memory://snapshot/model.sysml") (range (start 3 9) (end 3 15)) (role Declaration))
          (declaration (range (start 3 4) (end 3 22)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Mot") (target unresolved))
          (typing (outcome unresolved))
          (effective-typing (outcome unresolved))
        )
      )
      (reference-kind featureTyping)
      (referenced (status unresolved))
    )
    (completion-imports (status resolved) (import (target "A::Motor") (referent "A::Motor") (spelling "Motor") (namespace "Machine") (range (start 3 18) (end 3 21))) (safe true) (import (target "B::Motor") (referent "B::Motor") (spelling "Motor") (namespace "Machine") (range (start 3 18) (end 3 21))) (safe true))
    (missing-imports (status resolved) (repaired) (remaining (Unavailable (range (start 3 18) (end 3 21))) (Ambiguous (range (start 4 17) (end 4 22)))))
  )
  (document-symbols (document "memory://snapshot/model.sysml")
    (status resolved)
    (symbol (kind "Package") (name "A") (qualified-name "A") (location (document "memory://snapshot/model.sysml") (range (start 0 8) (end 0 9)) (role Declaration)) (declaration (range (start 0 0) (end 0 29))))
    (symbol (kind "PartDefinition") (name "Motor") (qualified-name "A::Motor") (location (document "memory://snapshot/model.sysml") (range (start 0 21) (end 0 26)) (role Declaration)) (declaration (range (start 0 12) (end 0 27))))
    (symbol (kind "Package") (name "B") (qualified-name "B") (location (document "memory://snapshot/model.sysml") (range (start 1 8) (end 1 9)) (role Declaration)) (declaration (range (start 1 0) (end 1 29))))
    (symbol (kind "PartDefinition") (name "Motor") (qualified-name "B::Motor") (location (document "memory://snapshot/model.sysml") (range (start 1 21) (end 1 26)) (role Declaration)) (declaration (range (start 1 12) (end 1 27))))
    (symbol (kind "Package") (name "Machine") (qualified-name "Machine") (location (document "memory://snapshot/model.sysml") (range (start 2 8) (end 2 15)) (role Declaration)) (declaration (range (start 2 0) (end 5 1))))
    (symbol (kind "PartUsage") (name "engine") (qualified-name "Machine::engine") (location (document "memory://snapshot/model.sysml") (range (start 3 9) (end 3 15)) (role Declaration)) (declaration (range (start 3 4) (end 3 22))))
    (symbol (kind "PartUsage") (name "other") (qualified-name "Machine::other") (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 14)) (role Declaration)) (declaration (range (start 4 4) (end 4 23))))
  )
)
~~~
