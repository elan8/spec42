# META
~~~ini
description=Private namespace imports remain locally visible and are not re-exported
type=multi
~~~
# SOURCE
## Source.sysml
~~~sysml
package Source {
    part def Thing;
}
~~~
## Middle.sysml
~~~sysml
package Middle {
    private import Source::*;
    part local : Thing;
}
~~~
## Client.sysml
~~~sysml
package Client {
    import Middle::*;
    part missing : Thing;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "Source.sysml"
    (diagnostics
    )
  )
  (document "Middle.sysml"
    (diagnostics
    )
  )
  (document "Client.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 4) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 19) (end 2 24))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "cecb76cb1e5e6152b3feb0c653a5d7f9b606abc4f863888a9b492ab7fdba285d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Client"))) (kind "package") (name "Client") (declared-name "Client"))
    (element (id (node (document "d0") (qualified-name "Client::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Client"))) (authored (membership (kind Import) (import (reference "Middle::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Client::missing"))) (kind "part") (name "missing") (declared-name "missing") (parent (node (document "d0") (qualified-name "Client"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thing")))))
    (element (id (node (document "d1") (qualified-name "Middle"))) (kind "package") (name "Middle") (declared-name "Middle"))
    (element (id (node (document "d1") (qualified-name "Middle::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d1") (qualified-name "Middle"))) (authored (membership (kind Import) (visibility "private") (import (reference "Source::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d1") (qualified-name "Middle::local"))) (kind "part") (name "local") (declared-name "local") (parent (node (document "d1") (qualified-name "Middle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Thing")))))
    (element (id (node (document "d2") (qualified-name "Source"))) (kind "package") (name "Source") (declared-name "Source"))
    (element (id (node (document "d2") (qualified-name "Source::Thing"))) (kind "part def") (name "Thing") (declared-name "Thing") (parent (node (document "d2") (qualified-name "Source"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Client::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Middle::*") (outcome (status resolved) (target (node (document "d1") (qualified-name "Middle")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "Client::missing"))) (kind featureTyping) (ordinal 0)) (authored-target "Thing") (outcome (status unresolved)))
    (reference (id (source (node (document "d1") (qualified-name "Middle::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Source::*") (outcome (status resolved) (target (node (document "d2") (qualified-name "Source")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d1") (qualified-name "Middle::local"))) (kind featureTyping) (ordinal 0)) (authored-target "Thing") (outcome (status resolved) (target (node (document "d2") (qualified-name "Source::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d1") (qualified-name "Middle::local"))) (target (node (document "d2") (qualified-name "Source::Thing"))) (provenance authored) (authored-reference (source (node (document "d1") (qualified-name "Middle::local"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 19) (end 2 24)) (probe (position 2 19))
      (reference
        (source (document "d0") (qualified-name "Client::missing"))
        (kind featureTyping) (ordinal 0) (authored-target "Thing")
        (range (start 2 19) (end 2 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 11) (end 1 17)) (probe (position 1 11))
      (reference
        (source (document "d0") (qualified-name "Client::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Middle::*")
        (range (start 1 11) (end 1 17))
        (outcome (status resolved)
          (target (document "d1") (qualified-name "Middle") (range (start 0 0) (end 0 72)))
        )
      )
    )
  )
  (document "d1"
    (query (range (start 2 17) (end 2 22)) (probe (position 2 17))
      (reference
        (source (document "d1") (qualified-name "Middle::local"))
        (kind featureTyping) (ordinal 0) (authored-target "Thing")
        (range (start 2 17) (end 2 22))
        (outcome (status resolved)
          (target (document "d2") (qualified-name "Source::Thing") (range (start 1 4) (end 1 19)))
        )
      )
    )
    (query (range (start 1 19) (end 1 25)) (probe (position 1 19))
      (reference
        (source (document "d1") (qualified-name "Middle::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Source::*")
        (range (start 1 19) (end 1 25))
        (outcome (status resolved)
          (target (document "d2") (qualified-name "Source") (range (start 0 0) (end 0 38)))
        )
      )
    )
  )
)
~~~
