# META
~~~ini
description=A document the parser could only partly recover publishes recovery rather than an empty inspection
type=file
~~~
# SOURCE
~~~sysml
package Salvaged {
    part def Known;
}

package Damaged {
    part def Half :>
    part orphan : Known;
~~~
# EDITOR QUERIES
~~~ini
probe recovery.md 1 13
probe recovery.md 5 13
probe recovery.md 6 18
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/recovery.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_body_or_semicolon")
        (source "parser")
        (range (start 5 4) (end 6 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 18) (end 6 23))
      )
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "parser")
        (range (start 6 24) (end 6 24))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:73d251941fc600f51d31bee8e470a6ffbba8bff9f8c5ccf0967193878ebf17cf"))
  (declarations
    (declaration (id (node (document "memory://snapshot/recovery.md") (qualified-name "Damaged"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recovery.md") (qualified-name "Damaged::orphan"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Known")))))
    (declaration (id (node (document "memory://snapshot/recovery.md") (qualified-name "Salvaged"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recovery.md") (qualified-name "Salvaged::Known"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/recovery.md") (qualified-name "Damaged::orphan"))) (kind featureTyping) (ordinal 0))
      (authored-target "Known")
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
  (query (document "memory://snapshot/recovery.md") (range (start 6 18) (end 6 23)) (probe (position 6 18))
    (reference (id (source (node (document "memory://snapshot/recovery.md") (qualified-name "Damaged::orphan"))) (kind featureTyping) (ordinal 0) (authored-target "Known")
      (outcome (status unresolved)))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/recovery.md") (position 1 13)
    (target (status incomplete) (candidate (name "Known") (location (document "memory://snapshot/recovery.md") (range (start 1 13) (end 1 18)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/recovery.md") (range (start 1 13) (end 1 18)) (role Declaration))))
    (rename (status incomplete))
    (visible-members (candidates (member (name "Damaged") (qualified-name "Damaged") (kind "Package")) (member (name "Known") (qualified-name "Salvaged::Known") (kind "PartDefinition")) (member (name "Salvaged") (qualified-name "Salvaged") (kind "Package"))))
    (inspection
      (status incomplete)
      (containing
        (element (kind "PartDefinition")
          (name "Known")
          (qualified-name "Salvaged::Known")
          (location (document "memory://snapshot/recovery.md") (range (start 1 13) (end 1 18)) (role Declaration))
          (declaration (range (start 1 4) (end 1 19)))
          (membership (kind owning) (visibility public) (provenance default))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/recovery.md") (position 5 13)
    (target (status unresolved))
    (rename (status incomplete))
    (visible-members (candidates (member (name "Damaged") (qualified-name "Damaged") (kind "Package")) (member (name "Salvaged") (qualified-name "Salvaged") (kind "Package")) (member (name "orphan") (qualified-name "Damaged::orphan") (kind "PartUsage"))))
    (inspection
      (status incomplete)
      (containing
        (element (kind "Package")
          (name "Damaged")
          (qualified-name "Damaged")
          (location (document "memory://snapshot/recovery.md") (range (start 4 8) (end 4 15)) (role Declaration))
          (declaration (range (start 4 0) (end 6 24)))
          (membership (kind owning) (visibility public) (provenance default))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/recovery.md") (position 6 18)
    (target (status unresolved))
    (rename (status incomplete))
    (visible-members (candidates (member (name "Damaged") (qualified-name "Damaged") (kind "Package")) (member (name "Salvaged") (qualified-name "Salvaged") (kind "Package")) (member (name "orphan") (qualified-name "Damaged::orphan") (kind "PartUsage"))))
    (inspection
      (status incomplete)
      (containing
        (element (kind "PartUsage")
          (name "orphan")
          (qualified-name "Damaged::orphan")
          (location (document "memory://snapshot/recovery.md") (range (start 6 9) (end 6 15)) (role Declaration))
          (declaration (range (start 6 4) (end 6 24)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Known") (target unresolved))
          (typing (outcome unresolved))
          (effective-typing (outcome unresolved))
        )
      )
      (reference-kind featureTyping)
      (referenced (status unresolved))
    )
  )
  (document-symbols (document "memory://snapshot/recovery.md")
    (status incomplete)
    (symbol (kind "Package") (name "Salvaged") (qualified-name "Salvaged") (location (document "memory://snapshot/recovery.md") (range (start 0 8) (end 0 16)) (role Declaration)) (declaration (range (start 0 0) (end 2 1))))
    (symbol (kind "PartDefinition") (name "Known") (qualified-name "Salvaged::Known") (location (document "memory://snapshot/recovery.md") (range (start 1 13) (end 1 18)) (role Declaration)) (declaration (range (start 1 4) (end 1 19))))
    (symbol (kind "Package") (name "Damaged") (qualified-name "Damaged") (location (document "memory://snapshot/recovery.md") (range (start 4 8) (end 4 15)) (role Declaration)) (declaration (range (start 4 0) (end 6 24))))
    (symbol (kind "PartUsage") (name "orphan") (qualified-name "Damaged::orphan") (location (document "memory://snapshot/recovery.md") (range (start 6 9) (end 6 15)) (role Declaration)) (declaration (range (start 6 4) (end 6 24))))
  )
)
~~~
