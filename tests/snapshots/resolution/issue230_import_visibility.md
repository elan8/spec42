# META
~~~ini
description=Issue 230: The pinned Import production requires explicit visibility; a malformed bare import preserves later explicit imports and declarations.
type=file
~~~
# SOURCE
~~~sysml
package Lib { part def Thing; }
package Imports {
    import Lib::*;
    private import Lib::*;
    part retained : Thing;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/issue230_import_visibility.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_import_visibility")
        (source "parser")
        (range (start 2 4) (end 2 10))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/issue230_import_visibility.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_import_visibility")
        (source "parser")
        (range (start 2 4) (end 2 10))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:c23ec1ecd6a527ea983b4f94f59f62f22d60991c6b3c5669bc93dd407b1eb647"))
  (declarations
    (declaration (id (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Imports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_import_visibility.md") (path (named (kind package) (name "Imports")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Lib") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/issue230_import_visibility.md") (path (named (kind package) (name "Imports")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Lib") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Imports::retained"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/issue230_import_visibility.md") (path (named (kind package) (name "Imports")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib")))))
    (reference (id (source (node (document "memory://snapshot/issue230_import_visibility.md") (path (named (kind package) (name "Imports")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib")))))
    (reference (id (source (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Imports::retained"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Imports::retained"))) (target (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Imports::retained"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Imports::retained")))
      (type (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib::Thing")))
      (subtype (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Imports::retained")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/issue230_import_visibility.md") (range (start 2 11) (end 2 17)) (probe (position 2 11))
    (reference (id (source (node (document "memory://snapshot/issue230_import_visibility.md") (path (named (kind package) (name "Imports")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib")))))
    )
  )
  (query (document "memory://snapshot/issue230_import_visibility.md") (range (start 3 19) (end 3 25)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/issue230_import_visibility.md") (path (named (kind package) (name "Imports")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib")))))
    )
  )
  (query (document "memory://snapshot/issue230_import_visibility.md") (range (start 4 20) (end 4 25)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Imports::retained"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_import_visibility.md") (qualified-name "Lib::Thing")))))
    )
  )
)
~~~
