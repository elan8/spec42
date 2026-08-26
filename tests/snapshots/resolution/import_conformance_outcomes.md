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
    import Source::*;
    import Source::Item::*;
    import Missing::*;
    import Source [ 1 ];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/import_conformance_outcomes.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 5 11) (end 5 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 11) (end 6 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 11) (end 7 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 8 11) (end 8 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:95c4b81d2bb89e903f70db28c07a645888253bfd8bde375d5427749969d0b849"))
  (declarations
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (qualified-name "Client"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Source") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Source::Item") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Missing") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (filterImport (reference "Source") (import (shape filtered-namespace) (recursive false))))))
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
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/import_conformance_outcomes.md") (range (start 5 11) (end 5 20)) (probe (position 5 11))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/import_conformance_outcomes.md") (qualified-name "Source")))))
    )
  )
  (query (document "memory://snapshot/import_conformance_outcomes.md") (range (start 6 11) (end 6 26)) (probe (position 6 11))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Source::Item")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/import_conformance_outcomes.md") (range (start 7 11) (end 7 21)) (probe (position 7 11))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/import_conformance_outcomes.md") (range (start 8 11) (end 8 23)) (probe (position 8 11))
    (reference (id (source (node (document "memory://snapshot/import_conformance_outcomes.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 3))))) (kind filterImport) (ordinal 0) (authored-target "Source")
      (outcome (status unsupported)))
    )
  )
)
~~~
