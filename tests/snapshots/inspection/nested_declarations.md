# META
~~~ini
description=Inspection picks the innermost declaration containing a position across nested bodies
type=file
~~~
# SOURCE
~~~sysml
package Outer {
    part def Vehicle {
        part engine : Vehicle {
            attribute displacement;
        }
    }
}
~~~
# EDITOR QUERIES
~~~ini
probe nested_declarations.md 0 8
probe nested_declarations.md 1 13
probe nested_declarations.md 2 13
probe nested_declarations.md 3 22
probe nested_declarations.md 2 24
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/nested_declarations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c834da5c041e9d100f8b0c4614201de321c22dcd00582f898546baf6198db9bd") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine::displacement"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine"))) (target (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine"))) (target (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine::displacement"))) (target (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle")))
      (subtype (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine")))
      (featured-by (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle")))
      (type (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine::displacement")))
      (featured-by (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/nested_declarations.md") (range (start 2 22) (end 2 29)) (probe (position 2 22))
    (reference (id (source (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/nested_declarations.md") (qualified-name "Outer::Vehicle")))))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/nested_declarations.md") (position 0 8)
    (target (status resolved) (candidate (name "Outer") (location (document "memory://snapshot/nested_declarations.md") (range (start 0 8) (end 0 13)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/nested_declarations.md") (range (start 0 8) (end 0 13)) (role Declaration))))
    (rename (status ready) (name "Outer") (range (start 0 8) (end 0 13)) (occurrences 1))
    (visible-members (candidates (member (name "Outer") (qualified-name "Outer") (kind "Package")) (member (name "Vehicle") (qualified-name "Outer::Vehicle") (kind "PartDefinition"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "Package")
          (name "Outer")
          (qualified-name "Outer")
          (location (document "memory://snapshot/nested_declarations.md") (range (start 0 8) (end 0 13)) (role Declaration))
          (declaration (range (start 0 0) (end 6 1)))
          (membership (kind owning) (visibility public) (provenance default))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/nested_declarations.md") (position 1 13)
    (target (status resolved) (candidate (name "Vehicle") (location (document "memory://snapshot/nested_declarations.md") (range (start 1 13) (end 1 20)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/nested_declarations.md") (range (start 1 13) (end 1 20)) (role Declaration)) (location (document "memory://snapshot/nested_declarations.md") (range (start 2 22) (end 2 29)) (role Reference))))
    (rename (status ready) (name "Vehicle") (range (start 1 13) (end 1 20)) (occurrences 2))
    (visible-members (candidates (member (name "Outer") (qualified-name "Outer") (kind "Package")) (member (name "Vehicle") (qualified-name "Outer::Vehicle") (kind "PartDefinition")) (member (name "engine") (qualified-name "Outer::Vehicle::engine") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "Outer::Vehicle")
          (location (document "memory://snapshot/nested_declarations.md") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 5 5)))
          (membership (kind owning) (visibility public) (provenance default))
          (incoming (kind "typeFeaturing") (peer "Outer::Vehicle::engine") (provenance implied))
          (incoming (kind "typing") (peer "Outer::Vehicle::engine") (provenance authored))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/nested_declarations.md") (position 2 13)
    (target (status resolved) (candidate (name "engine") (location (document "memory://snapshot/nested_declarations.md") (range (start 2 13) (end 2 19)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/nested_declarations.md") (range (start 2 13) (end 2 19)) (role Declaration))))
    (rename (status ready) (name "engine") (range (start 2 13) (end 2 19)) (occurrences 1))
    (visible-members (candidates (member (name "Outer") (qualified-name "Outer") (kind "Package")) (member (name "Vehicle") (qualified-name "Outer::Vehicle") (kind "PartDefinition")) (member (name "displacement") (qualified-name "Outer::Vehicle::engine::displacement") (kind "AttributeUsage")) (member (name "engine") (qualified-name "Outer::Vehicle::engine") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "engine")
          (qualified-name "Outer::Vehicle::engine")
          (location (document "memory://snapshot/nested_declarations.md") (range (start 2 13) (end 2 19)) (role Declaration))
          (declaration (range (start 2 8) (end 4 9)))
          (membership (kind feature) (visibility private) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Vehicle") (target resolved))
          (relationship (kind "typeFeaturing") (provenance implied) (target resolved))
          (typing (outcome resolved) (target "Outer::Vehicle"))
          (effective-typing (outcome resolved) (type (qualified-name "Outer::Vehicle") (origin direct)))
          (inherited-feature (qualified-name "Outer::Vehicle::engine") (declared-in "Outer::Vehicle"))
          (incoming (kind "typeFeaturing") (peer "Outer::Vehicle::engine::displacement") (provenance implied))
          (outgoing (kind "typeFeaturing") (peer "Outer::Vehicle") (provenance implied))
          (outgoing (kind "typing") (peer "Outer::Vehicle") (provenance authored))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/nested_declarations.md") (position 3 22)
    (target (status resolved) (candidate (name "displacement") (location (document "memory://snapshot/nested_declarations.md") (range (start 3 22) (end 3 34)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/nested_declarations.md") (range (start 3 22) (end 3 34)) (role Declaration))))
    (rename (status ready) (name "displacement") (range (start 3 22) (end 3 34)) (occurrences 1))
    (visible-members (candidates (member (name "Outer") (qualified-name "Outer") (kind "Package")) (member (name "Vehicle") (qualified-name "Outer::Vehicle") (kind "PartDefinition")) (member (name "displacement") (qualified-name "Outer::Vehicle::engine::displacement") (kind "AttributeUsage")) (member (name "engine") (qualified-name "Outer::Vehicle::engine") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "AttributeUsage")
          (name "displacement")
          (qualified-name "Outer::Vehicle::engine::displacement")
          (location (document "memory://snapshot/nested_declarations.md") (range (start 3 22) (end 3 34)) (role Declaration))
          (declaration (range (start 3 12) (end 3 35)))
          (membership (kind feature) (visibility private) (provenance default))
          (relationship (kind "typeFeaturing") (provenance implied) (target resolved))
          (outgoing (kind "typeFeaturing") (peer "Outer::Vehicle::engine") (provenance implied))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/nested_declarations.md") (position 2 24)
    (target (status resolved) (candidate (name "Vehicle") (location (document "memory://snapshot/nested_declarations.md") (range (start 1 13) (end 1 20)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/nested_declarations.md") (range (start 1 13) (end 1 20)) (role Declaration)) (location (document "memory://snapshot/nested_declarations.md") (range (start 2 22) (end 2 29)) (role Reference))))
    (rename (status ready) (name "Vehicle") (range (start 2 22) (end 2 29)) (occurrences 2))
    (visible-members (candidates (member (name "Outer") (qualified-name "Outer") (kind "Package")) (member (name "Vehicle") (qualified-name "Outer::Vehicle") (kind "PartDefinition")) (member (name "displacement") (qualified-name "Outer::Vehicle::engine::displacement") (kind "AttributeUsage")) (member (name "engine") (qualified-name "Outer::Vehicle::engine") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "engine")
          (qualified-name "Outer::Vehicle::engine")
          (location (document "memory://snapshot/nested_declarations.md") (range (start 2 13) (end 2 19)) (role Declaration))
          (declaration (range (start 2 8) (end 4 9)))
          (membership (kind feature) (visibility private) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Vehicle") (target resolved))
          (relationship (kind "typeFeaturing") (provenance implied) (target resolved))
          (typing (outcome resolved) (target "Outer::Vehicle"))
          (effective-typing (outcome resolved) (type (qualified-name "Outer::Vehicle") (origin direct)))
          (inherited-feature (qualified-name "Outer::Vehicle::engine") (declared-in "Outer::Vehicle"))
          (incoming (kind "typeFeaturing") (peer "Outer::Vehicle::engine::displacement") (provenance implied))
          (outgoing (kind "typeFeaturing") (peer "Outer::Vehicle") (provenance implied))
          (outgoing (kind "typing") (peer "Outer::Vehicle") (provenance authored))
        )
      )
      (referenced (status resolved)
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "Outer::Vehicle")
          (location (document "memory://snapshot/nested_declarations.md") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 5 5)))
          (membership (kind owning) (visibility public) (provenance default))
          (incoming (kind "typeFeaturing") (peer "Outer::Vehicle::engine") (provenance implied))
          (incoming (kind "typing") (peer "Outer::Vehicle::engine") (provenance authored))
        )
      )
    )
  )
  (document-symbols (document "memory://snapshot/nested_declarations.md")
    (status resolved)
    (symbol (kind "Package") (name "Outer") (qualified-name "Outer") (location (document "memory://snapshot/nested_declarations.md") (range (start 0 8) (end 0 13)) (role Declaration)) (declaration (range (start 0 0) (end 6 1))))
    (symbol (kind "PartDefinition") (name "Vehicle") (qualified-name "Outer::Vehicle") (location (document "memory://snapshot/nested_declarations.md") (range (start 1 13) (end 1 20)) (role Declaration)) (declaration (range (start 1 4) (end 5 5))))
    (symbol (kind "PartUsage") (name "engine") (qualified-name "Outer::Vehicle::engine") (location (document "memory://snapshot/nested_declarations.md") (range (start 2 13) (end 2 19)) (role Declaration)) (declaration (range (start 2 8) (end 4 9))))
    (symbol (kind "AttributeUsage") (name "displacement") (qualified-name "Outer::Vehicle::engine::displacement") (location (document "memory://snapshot/nested_declarations.md") (range (start 3 22) (end 3 34)) (role Declaration)) (declaration (range (start 3 12) (end 3 35))))
  )
)
~~~
