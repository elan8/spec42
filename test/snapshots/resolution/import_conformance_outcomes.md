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
  (document "import_conformance_outcomes.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 5 4) (end 5 21))
      )
      (diagnostic
        (severity warning)
        (code "import_kind_mismatch")
        (source "semantic")
        (range (start 6 11) (end 6 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 11) (end 7 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 8 11) (end 8 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "21ecdae3ff9fabec9c355417ab43451897401006e557e46a9c1609c2532bff24") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Client"))) (kind "package") (name "Client") (declared-name "Client"))
    (element (id (node (document "d0") (qualified-name "Client::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Client"))) (authored (membership (kind Import) (import (reference "Source::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Client::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Client"))) (authored (membership (kind Import) (import (reference "Source::Item::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Client::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Client"))) (authored (membership (kind Import) (import (reference "Missing::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Client::Source"))) (kind "import") (name "Source") (declared-name "Source") (parent (node (document "d0") (qualified-name "Client"))) (authored (membership (kind Import) (import (reference "Source") (origin Import) (shape FilteredNamespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Source"))) (kind "package") (name "Source") (declared-name "Source"))
    (element (id (node (document "d0") (qualified-name "Source::Inner"))) (kind "package") (name "Inner") (declared-name "Inner") (parent (node (document "d0") (qualified-name "Source"))))
    (element (id (node (document "d0") (qualified-name "Source::Item"))) (kind "part def") (name "Item") (declared-name "Item") (parent (node (document "d0") (qualified-name "Source"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Client::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Source::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "Source")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "Client::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Source::Item::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "Source::Item")))) (import (origin import) (shape namespace) (recursive false) (conformance namespace-kind-mismatch (actual-kind "part def"))))
    (reference (id (source (node (document "d0") (qualified-name "Client::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "Missing::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Client::Source"))) (kind namespaceImport) (ordinal 0)) (authored-target "Source") (outcome (status unsupported-filtered)) (import (origin import) (shape filtered-namespace) (recursive false) (conformance not-checked-unsupported-filtered)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 5 11) (end 5 17)) (probe (position 5 11))
      (reference
        (source (document "d0") (qualified-name "Client::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Source::*")
        (range (start 5 11) (end 5 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Source") (range (start 0 0) (end 0 56)))
        )
      )
    )
    (query (range (start 8 11) (end 8 17)) (probe (position 8 11))
      (reference
        (source (document "d0") (qualified-name "Client::Source"))
        (kind namespaceImport) (ordinal 0) (authored-target "Source")
        (range (start 8 11) (end 8 17))
        (outcome (status unsupported-filtered))
      )
    )
    (query (range (start 7 11) (end 7 18)) (probe (position 7 11))
      (reference
        (source (document "d0") (qualified-name "Client::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "Missing::*")
        (range (start 7 11) (end 7 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 11) (end 6 23)) (probe (position 6 11))
      (reference
        (source (document "d0") (qualified-name "Client::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Source::Item::*")
        (range (start 6 11) (end 6 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Source::Item") (range (start 2 4) (end 2 18)))
        )
      )
    )
  )
)
~~~
