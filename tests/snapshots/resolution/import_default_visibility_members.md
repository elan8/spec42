# META
~~~ini
description=Members without authored visibility of a part usage or part definition are public, so `import <usage>::*` and `import <Definition>::*` bring them into scope (spec42 issue #231)
type=file
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package P {
    part def Engine;
    part def Wheel;
    part def Car;
    part vehicle : Car { part engine : Engine; }
    part def Vehicle { part wheel : Wheel; }
    package FromUsage {
        import vehicle::*;
        part e : Engine :> engine;
    }
    package FromDefinition {
        import Vehicle::*;
        part w : Wheel :> wheel;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/import_default_visibility_members.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:555a3a610b47e1629bdc95ba6d10e7bc86605efb4942081189b0e01a7f7e9cbc"))
  (declarations
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Car"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (path (named (kind package) (name "P")) (named (kind package) (name "FromDefinition")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Vehicle") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")) (subsetting (reference "wheel")))))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (path (named (kind package) (name "P")) (named (kind package) (name "FromUsage")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "vehicle") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")) (subsetting (reference "engine")))))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Car")))))
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (path (named (kind package) (name "P")) (named (kind package) (name "FromDefinition")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w"))) (kind subsetting) (ordinal 0))
      (authored-target "wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel")))))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (path (named (kind package) (name "P")) (named (kind package) (name "FromUsage")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")))))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e"))) (kind subsetting) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine")))))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Car")))))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w"))) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w"))) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e"))) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e"))) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel"))) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle"))) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Car"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine"))) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel"))) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine"))) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Car")))
      (subtype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")))
      (subtype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e")) (scopes any))
      (subtype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w")))
      (type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")) (source direct))
      (effective-type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")) (source inherited) (from (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel"))))
      (supertype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel")) (scopes any feature))
      (supertype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e")))
      (type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")) (source direct))
      (effective-type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")) (source inherited) (from (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine"))))
      (supertype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")) (scopes any))
      (supertype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel")))
      (featured-by (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle")))
      (type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")) (scopes any))
      (subtype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")))
      (subtype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w")) (scopes any))
      (subtype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle")))
      (type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Car")) (provenance authored))
      (effective-type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Car")) (source direct))
      (supertype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Car")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine")))
      (featured-by (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle")))
      (type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")) (scopes any))
      (subtype (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/import_default_visibility_members.md") (range (start 11 15) (end 11 25)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (path (named (kind package) (name "P")) (named (kind package) (name "FromDefinition")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/import_default_visibility_members.md") (range (start 12 17) (end 12 22)) (probe (position 12 17))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")))))
    )
  )
  (query (document "memory://snapshot/import_default_visibility_members.md") (range (start 12 26) (end 12 31)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromDefinition::w"))) (kind subsetting) (ordinal 0) (authored-target "wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel")))))
    )
  )
  (query (document "memory://snapshot/import_default_visibility_members.md") (range (start 7 15) (end 7 25)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (path (named (kind package) (name "P")) (named (kind package) (name "FromUsage")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle")))))
    )
  )
  (query (document "memory://snapshot/import_default_visibility_members.md") (range (start 8 17) (end 8 23)) (probe (position 8 17))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")))))
    )
  )
  (query (document "memory://snapshot/import_default_visibility_members.md") (range (start 8 27) (end 8 33)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::FromUsage::e"))) (kind subsetting) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine")))))
    )
  )
  (query (document "memory://snapshot/import_default_visibility_members.md") (range (start 5 36) (end 5 41)) (probe (position 5 36))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Vehicle::wheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Wheel")))))
    )
  )
  (query (document "memory://snapshot/import_default_visibility_members.md") (range (start 4 19) (end 4 22)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Car")))))
    )
  )
  (query (document "memory://snapshot/import_default_visibility_members.md") (range (start 4 39) (end 4 45)) (probe (position 4 39))
    (reference (id (source (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_default_visibility_members.md") (qualified-name "P::Engine")))))
    )
  )
)
~~~
