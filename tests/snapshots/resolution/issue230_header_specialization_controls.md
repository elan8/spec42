# META
~~~ini
description=Item subsettings, KerML reference subsettings, viewpoint subsettings and subject redefinitions retain their canonical targets; keyword names use unrestricted spelling.
type=file
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package Items {
    item def Item;
    item objects : Item;
    item things : Item :> objects;
}
package Features { classifier C { feature base; feature 'alias' references base; } }
package Viewpoints { viewpoint base; viewpoint 'derived' :> base; }
package Subjects {
    part def Vehicle;
    requirement def R { subject vehicle : Vehicle; }
    requirement def S :> R { subject subVehicle :>> vehicle; }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "Items::things") (target "Items::objects") (provenance authored) (outcome resolved))
  (relationship (kind subsetting) (source "Viewpoints::derived") (target "Viewpoints::base") (provenance authored) (outcome resolved))
  (relationship (kind redefinition) (source "Subjects::S::subVehicle") (target "Subjects::R::vehicle") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/issue230_header_specialization_controls.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9b14a09cd1a73840e7122400c0ed85a7460836a6f61e609b0adf5a624a5e7170"))
  (declarations
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::alias"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "base")))))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Item")))))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Item")) (subsetting (reference "objects")))))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "R")))))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S::subVehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::base"))) (kind viewpoint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::derived"))) (kind viewpoint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::alias"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::base")))))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects"))) (kind featureTyping) (ordinal 0))
      (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")))))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things"))) (kind featureTyping) (ordinal 0))
      (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")))))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things"))) (kind subsetting) (ordinal 0))
      (authored-target "objects")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects")))))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S"))) (kind specialization) (ordinal 0))
      (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R")))))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S::subVehicle"))) (kind redefinition) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::derived"))) (kind subsetting) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::base")))))
  )
  (relationships
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::alias"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::alias"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S::subVehicle"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S::subVehicle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::derived"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::derived"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::alias"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::base"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S::subVehicle"))) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::alias")))
      (featured-by (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C")))
      (supertype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::base")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::base")))
      (featured-by (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C")))
      (subtype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::alias")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")))
      (subtype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects")) (scopes any))
      (subtype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects")))
      (type (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")) (source direct))
      (supertype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")) (scopes any))
      (subtype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things")))
      (type (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")) (source direct))
      (effective-type (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects"))))
      (supertype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R")))
      (subtype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle")))
      (featured-by (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R")))
      (type (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::Vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S::subVehicle")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S")))
      (supertype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S::subVehicle")))
      (featured-by (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S")))
      (effective-type (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::Vehicle")) (source inherited) (from (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle"))))
      (supertype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle")) (scopes any feature))
      (supertype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::Vehicle")))
      (subtype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::base")))
      (subtype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::derived")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::derived")))
      (supertype (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::base")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/issue230_header_specialization_controls.md") (range (start 5 75) (end 5 79)) (probe (position 5 75))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::alias"))) (kind referenceSubsetting) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Features::C::base")))))
    )
  )
  (query (document "memory://snapshot/issue230_header_specialization_controls.md") (range (start 2 19) (end 2 23)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects"))) (kind featureTyping) (ordinal 0) (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")))))
    )
  )
  (query (document "memory://snapshot/issue230_header_specialization_controls.md") (range (start 3 18) (end 3 22)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things"))) (kind featureTyping) (ordinal 0) (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::Item")))))
    )
  )
  (query (document "memory://snapshot/issue230_header_specialization_controls.md") (range (start 3 26) (end 3 33)) (probe (position 3 26))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::things"))) (kind subsetting) (ordinal 0) (authored-target "objects")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Items::objects")))))
    )
  )
  (query (document "memory://snapshot/issue230_header_specialization_controls.md") (range (start 9 42) (end 9 49)) (probe (position 9 42))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/issue230_header_specialization_controls.md") (range (start 10 25) (end 10 26)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S"))) (kind specialization) (ordinal 0) (authored-target "R")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R")))))
    )
  )
  (query (document "memory://snapshot/issue230_header_specialization_controls.md") (range (start 10 52) (end 10 59)) (probe (position 10 52))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::S::subVehicle"))) (kind redefinition) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Subjects::R::vehicle")))))
    )
  )
  (query (document "memory://snapshot/issue230_header_specialization_controls.md") (range (start 6 60) (end 6 64)) (probe (position 6 60))
    (reference (id (source (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::derived"))) (kind subsetting) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_header_specialization_controls.md") (qualified-name "Viewpoints::base")))))
    )
  )
)
~~~
