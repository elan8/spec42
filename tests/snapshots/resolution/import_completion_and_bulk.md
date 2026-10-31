# META
~~~ini
description=Canonical import planning: import completion and bulk
type=file
libraries=none
~~~
# SOURCE
## model.sysml
~~~sysml
package Components {
    part def Motor;
    private part def Secret;
    port def MotorPort;
    alias Engine for Motor;
}
package Machine {
    part driveA : Mot;
    part driveB : Motor;
    part third : Motor;
    port connectionPoint : MotorPort;
    part unknown : Secret;
    part aliased : Engine;
}
~~~
# EDITOR QUERIES
~~~text
probe model.sysml 7 20 imports=Mot fix-imports
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
        (range (start 7 18) (end 7 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 18) (end 8 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 17) (end 9 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 27) (end 10 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 19) (end 11 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 19) (end 12 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:18520f68d662720ea4f3fa43ebae72b8d2fdd8ae6162886cd43405c184decff2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Components"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Engine"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Motor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Components::MotorPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Components")) (named (kind port-def) (name "MotorPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~MotorPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Secret"))) (kind part-def) (membership (kind owning) (visibility private)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::connectionPoint"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MotorPort")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveA"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Mot")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveB"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::third"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::unknown"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Secret")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Engine"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::connectionPoint"))) (kind featureTyping) (ordinal 0))
      (authored-target "MotorPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveA"))) (kind featureTyping) (ordinal 0))
      (authored-target "Mot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveB"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::third"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::unknown"))) (kind featureTyping) (ordinal 0))
      (authored-target "Secret")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Engine"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Engine"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Components")) (named (kind port-def) (name "MotorPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Components::MotorPort"))) (provenance implied))
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
  (query (document "memory://snapshot/model.sysml") (range (start 4 21) (end 4 26)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Engine"))) (kind aliasBinding) (ordinal 0) (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 19) (end 12 25)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 27) (end 10 36)) (probe (position 10 27))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::connectionPoint"))) (kind featureTyping) (ordinal 0) (authored-target "MotorPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 18) (end 7 21)) (probe (position 7 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveA"))) (kind featureTyping) (ordinal 0) (authored-target "Mot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 18) (end 8 23)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveB"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 17) (end 9 22)) (probe (position 9 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::third"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 19) (end 11 25)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::unknown"))) (kind featureTyping) (ordinal 0) (authored-target "Secret")
      (outcome (status unresolved)))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/model.sysml") (position 7 20)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "Components") (qualified-name "Components") (kind "Package")) (member (name "Machine") (qualified-name "Machine") (kind "Package")) (member (name "aliased") (qualified-name "Machine::aliased") (kind "PartUsage")) (member (name "connectionPoint") (qualified-name "Machine::connectionPoint") (kind "PortUsage")) (member (name "driveA") (qualified-name "Machine::driveA") (kind "PartUsage")) (member (name "driveB") (qualified-name "Machine::driveB") (kind "PartUsage")) (member (name "third") (qualified-name "Machine::third") (kind "PartUsage")) (member (name "unknown") (qualified-name "Machine::unknown") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "driveA")
          (qualified-name "Machine::driveA")
          (location (document "memory://snapshot/model.sysml") (range (start 7 9) (end 7 15)) (role Declaration))
          (declaration (range (start 7 4) (end 7 22)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Mot") (target unresolved))
          (typing (outcome unresolved))
          (effective-typing (outcome unresolved))
        )
      )
      (reference-kind featureTyping)
      (referenced (status unresolved))
    )
    (completion-imports (status resolved) (import (target "Components::Motor") (referent "Components::Motor") (spelling "Motor") (namespace "Machine") (range (start 7 18) (end 7 21))) (safe true))
    (missing-imports (status resolved) (import (target "Components::Engine") (referent "Components::Motor") (spelling "Engine") (namespace "Machine") (range (start 12 19) (end 12 25))) (import (target "Components::Motor") (referent "Components::Motor") (spelling "Motor") (namespace "Machine") (range (start 8 18) (end 8 23))) (import (target "Components::MotorPort") (referent "Components::MotorPort") (spelling "MotorPort") (namespace "Machine") (range (start 10 27) (end 10 36))) (repaired (range (start 8 18) (end 8 23)) (range (start 9 17) (end 9 22)) (range (start 10 27) (end 10 36)) (range (start 12 19) (end 12 25))) (remaining (Unavailable (range (start 7 18) (end 7 21))) (Unavailable (range (start 11 19) (end 11 25)))))
  )
  (document-symbols (document "memory://snapshot/model.sysml")
    (status resolved)
    (symbol (kind "Package") (name "Components") (qualified-name "Components") (location (document "memory://snapshot/model.sysml") (range (start 0 8) (end 0 18)) (role Declaration)) (declaration (range (start 0 0) (end 5 1))))
    (symbol (kind "PartDefinition") (name "Motor") (qualified-name "Components::Motor") (location (document "memory://snapshot/model.sysml") (range (start 1 13) (end 1 18)) (role Declaration)) (declaration (range (start 1 4) (end 1 19))))
    (symbol (kind "PartDefinition") (name "Secret") (qualified-name "Components::Secret") (location (document "memory://snapshot/model.sysml") (range (start 2 21) (end 2 27)) (role Declaration)) (declaration (range (start 2 4) (end 2 28))))
    (symbol (kind "PortDefinition") (name "MotorPort") (qualified-name "Components::MotorPort") (location (document "memory://snapshot/model.sysml") (range (start 3 13) (end 3 22)) (role Declaration)) (declaration (range (start 3 4) (end 3 23))))
    (symbol (kind "Alias") (name "Engine") (qualified-name "Components::Engine") (location (document "memory://snapshot/model.sysml") (range (start 4 10) (end 4 16)) (role Declaration)) (declaration (range (start 4 4) (end 4 27))))
    (symbol (kind "Package") (name "Machine") (qualified-name "Machine") (location (document "memory://snapshot/model.sysml") (range (start 6 8) (end 6 15)) (role Declaration)) (declaration (range (start 6 0) (end 13 1))))
    (symbol (kind "PartUsage") (name "driveA") (qualified-name "Machine::driveA") (location (document "memory://snapshot/model.sysml") (range (start 7 9) (end 7 15)) (role Declaration)) (declaration (range (start 7 4) (end 7 22))))
    (symbol (kind "PartUsage") (name "driveB") (qualified-name "Machine::driveB") (location (document "memory://snapshot/model.sysml") (range (start 8 9) (end 8 15)) (role Declaration)) (declaration (range (start 8 4) (end 8 24))))
    (symbol (kind "PartUsage") (name "third") (qualified-name "Machine::third") (location (document "memory://snapshot/model.sysml") (range (start 9 9) (end 9 14)) (role Declaration)) (declaration (range (start 9 4) (end 9 23))))
    (symbol (kind "PortUsage") (name "connectionPoint") (qualified-name "Machine::connectionPoint") (location (document "memory://snapshot/model.sysml") (range (start 10 9) (end 10 24)) (role Declaration)) (declaration (range (start 10 4) (end 10 37))))
    (symbol (kind "PartUsage") (name "unknown") (qualified-name "Machine::unknown") (location (document "memory://snapshot/model.sysml") (range (start 11 9) (end 11 16)) (role Declaration)) (declaration (range (start 11 4) (end 11 26))))
    (symbol (kind "PartUsage") (name "aliased") (qualified-name "Machine::aliased") (location (document "memory://snapshot/model.sysml") (range (start 12 9) (end 12 16)) (role Declaration)) (declaration (range (start 12 4) (end 12 26))))
  )
)
~~~
