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
# FORMAT
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
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "21ecdae3ff9fabec9c355417ab43451897401006e557e46a9c1609c2532bff24") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Client"))) (kind "package") (name "Client") (declared-name "Client") (range (start (line 4) (character 0)) (end (line 4) (character 116))))
    (element (id (node (document "d0") (qualified-name "Client::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 5) (character 4)) (end (line 5) (character 21))) (parent (node (document "d0") (qualified-name "Client"))) (authored (membership (kind Import) (import (reference "Source::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 5) (character 11)) (end (line 5) (character 17))))))
    (element (id (node (document "d0") (qualified-name "Client::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 4)) (end (line 6) (character 27))) (parent (node (document "d0") (qualified-name "Client"))) (authored (membership (kind Import) (import (reference "Source::Item::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 11)) (end (line 6) (character 23))))))
    (element (id (node (document "d0") (qualified-name "Client::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 4)) (end (line 7) (character 22))) (parent (node (document "d0") (qualified-name "Client"))) (authored (membership (kind Import) (import (reference "Missing::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 11)) (end (line 7) (character 18))))))
    (element (id (node (document "d0") (qualified-name "Client::Source"))) (kind "import") (name "Source") (declared-name "Source") (range (start (line 8) (character 4)) (end (line 8) (character 24))) (parent (node (document "d0") (qualified-name "Client"))) (authored (membership (kind Import) (import (reference "Source") (origin Import) (shape FilteredNamespace) (recursive false)) (import-range (start (line 8) (character 11)) (end (line 8) (character 17))))))
    (element (id (node (document "d0") (qualified-name "Source"))) (kind "package") (name "Source") (declared-name "Source") (range (start (line 0) (character 0)) (end (line 0) (character 56))))
    (element (id (node (document "d0") (qualified-name "Source::Inner"))) (kind "package") (name "Inner") (declared-name "Inner") (range (start (line 1) (character 4)) (end (line 1) (character 18))) (parent (node (document "d0") (qualified-name "Source"))))
    (element (id (node (document "d0") (qualified-name "Source::Item"))) (kind "part def") (name "Item") (declared-name "Item") (range (start (line 2) (character 4)) (end (line 2) (character 18))) (parent (node (document "d0") (qualified-name "Source"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Client::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Source::*") (range (start (line 5) (character 11)) (end (line 5) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Source")))))
    (reference (id (source (node (document "d0") (qualified-name "Client::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Source::Item::*") (range (start (line 6) (character 11)) (end (line 6) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Source::Item")))))
    (reference (id (source (node (document "d0") (qualified-name "Client::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "Missing::*") (range (start (line 7) (character 11)) (end (line 7) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Client::Source"))) (kind namespaceImport) (ordinal 0)) (authored-target "Source") (range (start (line 8) (character 11)) (end (line 8) (character 17))) (outcome (status unsupported-filtered)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
