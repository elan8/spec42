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
  (document "memory://snapshot/Client.sysml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 19) (end 2 24))
      )
    )
  )
  (document "memory://snapshot/Middle.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/Source.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ebdcccea47dfd55a5eb90d724967cbca0dee26cd6cd8588b2b5644d3a78becb9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/Client.sysml") (qualified-name "Client"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Client.sysml") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Middle") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/Client.sysml") (qualified-name "Client::missing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/Middle.sysml") (qualified-name "Middle"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Middle.sysml") (path (named (kind package) (name "Middle")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Source") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/Middle.sysml") (qualified-name "Middle::local"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/Source.sysml") (qualified-name "Source"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/Source.sysml") (qualified-name "Source::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/Client.sysml") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Middle")
      (outcome (status resolved) (target (node (document "memory://snapshot/Middle.sysml") (qualified-name "Middle")))))
    (reference (id (source (node (document "memory://snapshot/Client.sysml") (qualified-name "Client::missing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/Middle.sysml") (path (named (kind package) (name "Middle")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/Source.sysml") (qualified-name "Source")))))
    (reference (id (source (node (document "memory://snapshot/Middle.sysml") (qualified-name "Middle::local"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/Source.sysml") (qualified-name "Source::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/Middle.sysml") (qualified-name "Middle::local"))) (target (node (document "memory://snapshot/Source.sysml") (qualified-name "Source::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/Middle.sysml") (qualified-name "Middle::local"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/Middle.sysml") (qualified-name "Middle::local")))
      (supertype (node (document "memory://snapshot/Source.sysml") (qualified-name "Source::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/Client.sysml") (range (start 1 11) (end 1 20)) (probe (position 1 11))
    (reference (id (source (node (document "memory://snapshot/Client.sysml") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Middle")
      (outcome (status resolved) (target (node (document "memory://snapshot/Middle.sysml") (qualified-name "Middle")))))
    )
  )
  (query (document "memory://snapshot/Client.sysml") (range (start 2 19) (end 2 24)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/Client.sysml") (qualified-name "Client::missing"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/Middle.sysml") (range (start 1 19) (end 1 28)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/Middle.sysml") (path (named (kind package) (name "Middle")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/Source.sysml") (qualified-name "Source")))))
    )
  )
  (query (document "memory://snapshot/Middle.sysml") (range (start 2 17) (end 2 22)) (probe (position 2 17))
    (reference (id (source (node (document "memory://snapshot/Middle.sysml") (qualified-name "Middle::local"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/Source.sysml") (qualified-name "Source::Thing")))))
    )
  )
)
~~~
