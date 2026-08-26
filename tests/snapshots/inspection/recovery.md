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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:8b20fc341d5924a47d21bb56068d6414d73a0b180a89ca5ee44c79ddce620179") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/recovery.md") (qualified-name "Salvaged"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recovery.md") (qualified-name "Salvaged::Known"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
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
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/recovery.md") (position 1 13)
    (target (status incomplete) (candidate (name "Known") (location (document "memory://snapshot/recovery.md") (range (start 1 13) (end 1 18)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/recovery.md") (range (start 1 13) (end 1 18)) (role Declaration))))
    (rename (status incomplete))
    (visible-members (candidates (member (name "Known") (qualified-name "Salvaged::Known") (kind "PartDefinition")) (member (name "Salvaged") (qualified-name "Salvaged") (kind "Package"))))
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
    (visible-members (candidates (member (name "Salvaged") (qualified-name "Salvaged") (kind "Package"))))
    (inspection
      (status incomplete)
      (containing (status none))
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/recovery.md") (position 6 18)
    (target (status unresolved))
    (rename (status incomplete))
    (visible-members (candidates (member (name "Salvaged") (qualified-name "Salvaged") (kind "Package"))))
    (inspection
      (status incomplete)
      (containing (status none))
      (referenced (status none))
    )
  )
  (document-symbols (document "memory://snapshot/recovery.md")
    (status incomplete)
    (symbol (kind "Package") (name "Salvaged") (qualified-name "Salvaged") (location (document "memory://snapshot/recovery.md") (range (start 0 8) (end 0 16)) (role Declaration)) (declaration (range (start 0 0) (end 2 1))))
    (symbol (kind "PartDefinition") (name "Known") (qualified-name "Salvaged::Known") (location (document "memory://snapshot/recovery.md") (range (start 1 13) (end 1 18)) (role Declaration)) (declaration (range (start 1 4) (end 1 19))))
  )
)
~~~
