# META
~~~ini
description=Typed import conformance outcomes
type=file
~~~
# SOURCE
~~~sysml
package Source {
    package Inner;
    part def Item;
}
package Client {
    private import Source::*;
    private import Source::Item::*;
    private import Missing::*;
    private import Source [ 1 ];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/import_conformance_outcomes.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 5 19) (end 5 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 8 19) (end 8 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:50868d32184d574fb3f8e5a15af1c0b37dc532ec31fa2dc47cba89da7cb7b74e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (qualified-name "Client"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Source") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Source::Item") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Missing") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (filterImport (reference "Source") (import (shape filtered-namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (qualified-name "Source"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (qualified-name "Source::Inner"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (qualified-name "Source::Item"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_conformance_outcomes.md") (qualified-name "Source")))))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Source::Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_conformance_outcomes.md") (qualified-name "Source::Item")))))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 3))))) (kind filterImport) (ordinal 0))
      (authored-target "Source")
      (outcome (status unsupported)))
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
  (query (document "memory://snapshot/import_conformance_outcomes.md") (range (start 5 19) (end 5 28)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_conformance_outcomes.md") (qualified-name "Source")))))
    )
  )
  (query (document "memory://snapshot/import_conformance_outcomes.md") (range (start 6 19) (end 6 34)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Source::Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_conformance_outcomes.md") (qualified-name "Source::Item")))))
    )
  )
  (query (document "memory://snapshot/import_conformance_outcomes.md") (range (start 7 19) (end 7 29)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/import_conformance_outcomes.md") (range (start 8 19) (end 8 31)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 3))))) (kind filterImport) (ordinal 0) (authored-target "Source")
      (outcome (status unsupported)))
    )
  )
)
~~~
