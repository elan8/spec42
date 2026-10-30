# META
~~~ini
description=UTF-16 navigation and rename positions remain precise after multibyte text
type=file
~~~
# SOURCE
~~~sysml
package P { /* é😀 */ part def Motor; part vehicle : Motor; }
package Broken { /* é😀 */ part missing : Absent; }
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/unicode_editor_positions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 42) (end 1 48))
      )
    )
  )
)
~~~
# EDITOR QUERIES
~~~ini
probe unicode_editor_positions.md 0 31 hover
probe unicode_editor_positions.md 0 53 hover
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/unicode_editor_positions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 42) (end 1 48))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3df112c2ecc006b99b20b379222c15dc31f890c4dd1f671f82b7f9cb9bbd09d7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "Broken"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unicode_editor_positions.md") (path (named (kind package) (name "Broken")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "é😀 "))))
    (declaration (id (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "Broken::missing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Absent")))))
    (declaration (id (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unicode_editor_positions.md") (path (named (kind package) (name "P")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text "é😀 "))))
    (declaration (id (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::Motor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "Broken::missing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Absent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::Motor")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::vehicle"))) (target (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::Motor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::Motor")))
      (subtype (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::vehicle")))
      (type (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::Motor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::Motor")) (source direct))
      (supertype (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::Motor")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/unicode_editor_positions.md") (range (start 1 42) (end 1 48)) (probe (position 1 42))
    (reference (id (source (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "Broken::missing"))) (kind featureTyping) (ordinal 0) (authored-target "Absent")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 53) (end 0 58)) (probe (position 0 53))
    (reference (id (source (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/unicode_editor_positions.md") (qualified-name "P::Motor")))))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/unicode_editor_positions.md") (position 0 31)
    (target (status resolved) (candidate (name "Motor") (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 31) (end 0 36)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 31) (end 0 36)) (role Declaration)) (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 53) (end 0 58)) (role Reference))))
    (rename (status ready) (name "Motor") (range (start 0 31) (end 0 36)) (occurrences 2))
    (visible-members (candidates (member (name "Broken") (qualified-name "Broken") (kind "Package")) (member (name "Motor") (qualified-name "P::Motor") (kind "PartDefinition")) (member (name "P") (qualified-name "P") (kind "Package")) (member (name "vehicle") (qualified-name "P::vehicle") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartDefinition")
          (name "Motor")
          (qualified-name "P::Motor")
          (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 31) (end 0 36)) (role Declaration))
          (declaration (range (start 0 22) (end 0 37)))
          (membership (kind owning) (visibility public) (provenance default))
          (incoming (kind "typing") (peer "P::vehicle") (provenance authored))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/unicode_editor_positions.md") (position 0 53)
    (target (status resolved) (candidate (name "Motor") (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 31) (end 0 36)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 31) (end 0 36)) (role Declaration)) (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 53) (end 0 58)) (role Reference))))
    (rename (status ready) (name "Motor") (range (start 0 53) (end 0 58)) (occurrences 2))
    (visible-members (candidates (member (name "Broken") (qualified-name "Broken") (kind "Package")) (member (name "Motor") (qualified-name "P::Motor") (kind "PartDefinition")) (member (name "P") (qualified-name "P") (kind "Package")) (member (name "vehicle") (qualified-name "P::vehicle") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "vehicle")
          (qualified-name "P::vehicle")
          (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 43) (end 0 50)) (role Declaration))
          (declaration (range (start 0 38) (end 0 59)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Motor") (target resolved))
          (typing (outcome resolved) (target "P::Motor"))
          (effective-typing (outcome resolved) (type (qualified-name "P::Motor") (origin direct) (provenance authored)))
          (outgoing (kind "typing") (peer "P::Motor") (provenance authored))
        )
      )
      (reference-kind featureTyping)
      (referenced (status resolved)
        (element (kind "PartDefinition")
          (name "Motor")
          (qualified-name "P::Motor")
          (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 31) (end 0 36)) (role Declaration))
          (declaration (range (start 0 22) (end 0 37)))
          (membership (kind owning) (visibility public) (provenance default))
          (incoming (kind "typing") (peer "P::vehicle") (provenance authored))
        )
      )
    )
  )
  (document-symbols (document "memory://snapshot/unicode_editor_positions.md")
    (status resolved)
    (symbol (kind "Package") (name "P") (qualified-name "P") (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 8) (end 0 9)) (role Declaration)) (declaration (range (start 0 0) (end 0 61))))
    (symbol (kind "Comment") (qualified-name "P::") (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 12) (end 0 21)) (role Declaration)) (declaration (range (start 0 12) (end 0 21))))
    (symbol (kind "PartDefinition") (name "Motor") (qualified-name "P::Motor") (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 31) (end 0 36)) (role Declaration)) (declaration (range (start 0 22) (end 0 37))))
    (symbol (kind "PartUsage") (name "vehicle") (qualified-name "P::vehicle") (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 0 43) (end 0 50)) (role Declaration)) (declaration (range (start 0 38) (end 0 59))))
    (symbol (kind "Package") (name "Broken") (qualified-name "Broken") (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 1 8) (end 1 14)) (role Declaration)) (declaration (range (start 1 0) (end 1 51))))
    (symbol (kind "Comment") (qualified-name "Broken::") (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 1 17) (end 1 26)) (role Declaration)) (declaration (range (start 1 17) (end 1 26))))
    (symbol (kind "PartUsage") (name "missing") (qualified-name "Broken::missing") (location (document "memory://snapshot/unicode_editor_positions.md") (range (start 1 32) (end 1 39)) (role Declaration)) (declaration (range (start 1 27) (end 1 49))))
  )
)
~~~
# HOVER RESULTS
~~~sexpr
(hover-reports
  (probe (document "memory://snapshot/unicode_editor_positions.md") (position 0 31) (status available)
    (hover
      (identity (kind "part def") (name "Motor") (direct-types))
      (qualified-name "P::Motor")
      (destination (labels "Motor" "P::Motor") (uri "memory://snapshot/unicode_editor_positions.md") (position 0 31))
    )
  )
  (probe (document "memory://snapshot/unicode_editor_positions.md") (position 0 53) (status available)
    (hover
      (context (relation "Type of") (subject "P::vehicle"))
      (identity (kind "part def") (name "Motor") (direct-types))
      (qualified-name "P::Motor")
      (destination (labels "P::vehicle") (uri "memory://snapshot/unicode_editor_positions.md") (position 0 43))
      (destination (labels "Motor" "P::Motor") (uri "memory://snapshot/unicode_editor_positions.md") (position 0 31))
    )
  )
)
~~~
# HOVER MARKDOWN
## unicode_editor_positions.md:0:31
~~~markdown
`part def` **[Motor](memory://snapshot/unicode_editor_positions.md#L1)**

`P::Motor`
~~~
## unicode_editor_positions.md:0:53
~~~markdown
**Type of** [`P::vehicle`](memory://snapshot/unicode_editor_positions.md#L1)

`part def` **[Motor](memory://snapshot/unicode_editor_positions.md#L1)**

`P::Motor`
~~~
