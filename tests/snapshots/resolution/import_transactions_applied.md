# META
~~~ini
description=Applied import transactions bind repeated references, aliases and nested packages to their intended definitions
type=file
libraries=none
~~~
# SOURCE
## model.sysml
~~~sysml
package Catalog {
    part def Motor;
    port def Plug;
    alias Engine for Motor;
}
package Machine { // retain the package comment
    private import Catalog::Engine;
    private import Catalog::Motor;
    part driveA : Motor;
    part driveB : Motor;
    part aliased : Engine;
    package Inner {
        private import Catalog::Plug;
        port socket : Plug;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind feature_typing) (source "Machine::driveA") (target "Catalog::Motor") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "Machine::driveB") (target "Catalog::Motor") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "Machine::aliased") (target "Catalog::Motor") (provenance authored) (outcome resolved))
  (relationship (kind feature_typing) (source "Machine::Inner::socket") (target "Catalog::Plug") (provenance authored) (outcome resolved)))
~~~
# EDITOR QUERIES
~~~text
probe model.sysml 8 20 imports=Mot fix-imports
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1616dcb97503aebd487bd5defdfb959493c10aff701a3688ab915295af4b44c6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Engine"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Motor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Catalog")) (named (kind port-def) (name "Plug")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Plug") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Catalog::Engine") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Catalog::Motor") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::Inner"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind package) (name "Inner")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Catalog::Plug") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::Inner::socket"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Plug")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveA"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveB"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Engine"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Catalog::Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Engine")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Catalog::Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind package) (name "Inner")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Catalog::Plug")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::Inner::socket"))) (kind featureTyping) (ordinal 0))
      (authored-target "Plug")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveA"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveB"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Engine"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Engine"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::Inner::socket"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::Inner::socket"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveA"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveA"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveB"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Catalog")) (named (kind port-def) (name "Plug")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveA")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveB")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug")))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::Inner::socket")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::Inner::socket")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveA")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveB")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 3 21) (end 3 26)) (probe (position 3 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Engine"))) (kind aliasBinding) (ordinal 0) (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 19) (end 6 34)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Catalog::Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Engine")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 19) (end 7 33)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Catalog::Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 23) (end 12 36)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind package) (name "Inner")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Catalog::Plug")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 22) (end 13 26)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::Inner::socket"))) (kind featureTyping) (ordinal 0) (authored-target "Plug")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Plug")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 19) (end 10 25)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::aliased"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 18) (end 8 23)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveA"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 18) (end 9 23)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::driveB"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/model.sysml") (position 8 20)
    (target (status resolved) (candidate (name "Motor") (location (document "memory://snapshot/model.sysml") (range (start 1 13) (end 1 18)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/model.sysml") (range (start 1 13) (end 1 18)) (role Declaration)) (location (document "memory://snapshot/model.sysml") (range (start 3 21) (end 3 26)) (role Reference)) (location (document "memory://snapshot/model.sysml") (range (start 7 28) (end 7 33)) (role Reference)) (location (document "memory://snapshot/model.sysml") (range (start 8 18) (end 8 23)) (role Reference)) (location (document "memory://snapshot/model.sysml") (range (start 9 18) (end 9 23)) (role Reference)) (location (document "memory://snapshot/model.sysml") (range (start 10 19) (end 10 25)) (role Reference))))
    (rename (status ready) (name "Motor") (range (start 8 18) (end 8 23)) (occurrences 6))
    (visible-members (candidates (member (name "Catalog") (qualified-name "Catalog") (kind "Package")) (member (name "Engine") (qualified-name "Catalog::Engine") (kind "Alias")) (member (name "Inner") (qualified-name "Machine::Inner") (kind "Package")) (member (name "Machine") (qualified-name "Machine") (kind "Package")) (member (name "Motor") (qualified-name "Catalog::Motor") (kind "PartDefinition")) (member (name "aliased") (qualified-name "Machine::aliased") (kind "PartUsage")) (member (name "driveA") (qualified-name "Machine::driveA") (kind "PartUsage")) (member (name "driveB") (qualified-name "Machine::driveB") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "driveA")
          (qualified-name "Machine::driveA")
          (location (document "memory://snapshot/model.sysml") (range (start 8 9) (end 8 15)) (role Declaration))
          (declaration (range (start 8 4) (end 8 24)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Motor") (target resolved))
          (typing (outcome resolved) (target "Catalog::Motor"))
          (effective-typing (outcome resolved) (type (qualified-name "Catalog::Motor") (origin direct) (provenance authored)))
          (outgoing (kind "typing") (peer "Catalog::Motor") (provenance authored))
        )
      )
      (reference-kind featureTyping)
      (referenced (status resolved)
        (element (kind "PartDefinition")
          (name "Motor")
          (qualified-name "Catalog::Motor")
          (location (document "memory://snapshot/model.sysml") (range (start 1 13) (end 1 18)) (role Declaration))
          (declaration (range (start 1 4) (end 1 19)))
          (membership (kind owning) (visibility public) (provenance default))
          (incoming (kind "aliasBinding") (peer "Catalog::Engine") (provenance authored))
          (incoming (kind "typing") (peer "Machine::driveA") (provenance authored))
          (incoming (kind "typing") (peer "Machine::driveB") (provenance authored))
          (incoming (kind "typing") (peer "Machine::aliased") (provenance authored))
          (incoming (kind "typing") (peer "Machine::aliased") (provenance implied))
        )
      )
    )
    (completion-imports (status resolved))
    (missing-imports (status resolved) (repaired) (remaining))
  )
  (document-symbols (document "memory://snapshot/model.sysml")
    (status resolved)
    (symbol (kind "Package") (name "Catalog") (qualified-name "Catalog") (location (document "memory://snapshot/model.sysml") (range (start 0 8) (end 0 15)) (role Declaration)) (declaration (range (start 0 0) (end 4 1))))
    (symbol (kind "PartDefinition") (name "Motor") (qualified-name "Catalog::Motor") (location (document "memory://snapshot/model.sysml") (range (start 1 13) (end 1 18)) (role Declaration)) (declaration (range (start 1 4) (end 1 19))))
    (symbol (kind "PortDefinition") (name "Plug") (qualified-name "Catalog::Plug") (location (document "memory://snapshot/model.sysml") (range (start 2 13) (end 2 17)) (role Declaration)) (declaration (range (start 2 4) (end 2 18))))
    (symbol (kind "Alias") (name "Engine") (qualified-name "Catalog::Engine") (location (document "memory://snapshot/model.sysml") (range (start 3 10) (end 3 16)) (role Declaration)) (declaration (range (start 3 4) (end 3 27))))
    (symbol (kind "Package") (name "Machine") (qualified-name "Machine") (location (document "memory://snapshot/model.sysml") (range (start 5 8) (end 5 15)) (role Declaration)) (declaration (range (start 5 0) (end 15 1))))
    (symbol (kind "Import") (qualified-name "Machine::") (location (document "memory://snapshot/model.sysml") (range (start 6 4) (end 6 35)) (role Declaration)) (declaration (range (start 6 4) (end 6 35))))
    (symbol (kind "Import") (qualified-name "Machine::") (location (document "memory://snapshot/model.sysml") (range (start 7 4) (end 7 34)) (role Declaration)) (declaration (range (start 7 4) (end 7 34))))
    (symbol (kind "PartUsage") (name "driveA") (qualified-name "Machine::driveA") (location (document "memory://snapshot/model.sysml") (range (start 8 9) (end 8 15)) (role Declaration)) (declaration (range (start 8 4) (end 8 24))))
    (symbol (kind "PartUsage") (name "driveB") (qualified-name "Machine::driveB") (location (document "memory://snapshot/model.sysml") (range (start 9 9) (end 9 15)) (role Declaration)) (declaration (range (start 9 4) (end 9 24))))
    (symbol (kind "PartUsage") (name "aliased") (qualified-name "Machine::aliased") (location (document "memory://snapshot/model.sysml") (range (start 10 9) (end 10 16)) (role Declaration)) (declaration (range (start 10 4) (end 10 26))))
    (symbol (kind "Package") (name "Inner") (qualified-name "Machine::Inner") (location (document "memory://snapshot/model.sysml") (range (start 11 12) (end 11 17)) (role Declaration)) (declaration (range (start 11 4) (end 14 5))))
    (symbol (kind "Import") (qualified-name "Machine::Inner::") (location (document "memory://snapshot/model.sysml") (range (start 12 8) (end 12 37)) (role Declaration)) (declaration (range (start 12 8) (end 12 37))))
    (symbol (kind "PortUsage") (name "socket") (qualified-name "Machine::Inner::socket") (location (document "memory://snapshot/model.sysml") (range (start 13 13) (end 13 19)) (role Declaration)) (declaration (range (start 13 8) (end 13 27))))
  )
)
~~~
