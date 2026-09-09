# META
~~~ini
description=A workspace package named like a standard-library anchor package (Parts) still namespace-imports
type=file
libraries=standard
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package Parts {
    part def Widget;
}
package UsesParts {
    private import Parts::*;
    part w : Widget;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/workspace_shadows_library_anchor_package.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:291c43e1c2f03317c23bafd1ff49584ba98c47ff0a7af766c525d4b92f0c5525") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts::Widget"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "UsesParts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (path (named (kind package) (name "UsesParts")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Parts") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "UsesParts::w"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Widget")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (path (named (kind package) (name "UsesParts")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts")))))
    (reference (id (source (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "UsesParts::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "Widget")
      (outcome (status resolved) (target (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts::Widget")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "UsesParts::w"))) (target (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts::Widget"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "UsesParts::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts::Widget"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "UsesParts::w"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts::Widget")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "UsesParts::w")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "UsesParts::w")))
      (type (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts::Widget")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts::Widget")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts::Widget")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (range (start 4 19) (end 4 27)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (path (named (kind package) (name "UsesParts")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Parts")
      (outcome (status resolved) (target (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts")))))
    )
  )
  (query (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (range (start 5 13) (end 5 19)) (probe (position 5 13))
    (reference (id (source (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "UsesParts::w"))) (kind featureTyping) (ordinal 0) (authored-target "Widget")
      (outcome (status resolved) (target (node (document "memory://snapshot/workspace_shadows_library_anchor_package.md") (qualified-name "Parts::Widget")))))
    )
  )
)
~~~
