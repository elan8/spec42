# META
~~~ini
description=Expose targets resolve, and a view with a body that exposes nothing says so
type=file
~~~
# SOURCE
~~~sysml
package Exposing {
    part def Engine;
    part engine : Engine;
    viewpoint def Concerns;
    viewpoint concerns : Concerns;

    view conforming {
        expose engine;
    }

    view exposesSomethingThatDoesNotResolve {
        expose missingElement;
    }

    view declaresABodyButExposesNothing {
        satisfy concerns;
    }

    view declaresNoBody;

    view exposesAWildcard {
        expose Exposing::*;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/view_expose.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "view_expose_unresolved")
        (source "semantic")
        (range (start 11 15) (end 11 29))
      )
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 14 4) (end 16 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:76e89459202e3f2cd1ca8b740b1a99b19783f8f117e46dba49ef7cbaf422dfdb") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Concerns"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::concerns"))) (kind viewpoint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Concerns")))))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::conforming"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "conforming")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "engine")))))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::declaresABodyButExposesNothing"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "declaresABodyButExposesNothing")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "concerns")))))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::declaresNoBody"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::exposesAWildcard"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesAWildcard")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Exposing")))))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::exposesSomethingThatDoesNotResolve"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesSomethingThatDoesNotResolve")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "missingElement")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::concerns"))) (kind featureTyping) (ordinal 0))
      (authored-target "Concerns")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Concerns")))))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "conforming")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::engine")))))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "declaresABodyButExposesNothing")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "concerns")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::concerns")))))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Engine")))))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesAWildcard")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Exposing")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing")))))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesSomethingThatDoesNotResolve")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "missingElement")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::concerns"))) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Concerns"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::concerns"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "conforming")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "conforming")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "declaresABodyButExposesNothing")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::concerns"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "declaresABodyButExposesNothing")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::engine"))) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesAWildcard")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesAWildcard")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "conforming")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::conforming"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "declaresABodyButExposesNothing")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::declaresABodyButExposesNothing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesAWildcard")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::exposesAWildcard"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesSomethingThatDoesNotResolve")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::exposesSomethingThatDoesNotResolve"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Concerns")))
      (subtype (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::concerns")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Engine")))
      (subtype (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::concerns")))
      (type (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Concerns")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Concerns")) (source direct))
      (supertype (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Concerns")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "conforming")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::conforming")))
    )
    (declaration (id (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "declaresABodyButExposesNothing")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::declaresABodyButExposesNothing")))
    )
    (declaration (id (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::engine")))
      (type (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesAWildcard")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::exposesAWildcard")))
    )
    (declaration (id (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesSomethingThatDoesNotResolve")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::exposesSomethingThatDoesNotResolve")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/view_expose.md") (range (start 4 25) (end 4 33)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::concerns"))) (kind featureTyping) (ordinal 0) (authored-target "Concerns")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Concerns")))))
    )
  )
  (query (document "memory://snapshot/view_expose.md") (range (start 7 15) (end 7 21)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "conforming")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::engine")))))
    )
  )
  (query (document "memory://snapshot/view_expose.md") (range (start 15 16) (end 15 24)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "declaresABodyButExposesNothing")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "concerns")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::concerns")))))
    )
  )
  (query (document "memory://snapshot/view_expose.md") (range (start 2 18) (end 2 24)) (probe (position 2 18))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing::Engine")))))
    )
  )
  (query (document "memory://snapshot/view_expose.md") (range (start 21 15) (end 21 26)) (probe (position 21 15))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesAWildcard")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Exposing")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_expose.md") (qualified-name "Exposing")))))
    )
  )
  (query (document "memory://snapshot/view_expose.md") (range (start 11 15) (end 11 29)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/view_expose.md") (path (named (kind package) (name "Exposing")) (named (kind view) (name "exposesSomethingThatDoesNotResolve")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "missingElement")
      (outcome (status unresolved)))
    )
  )
)
~~~
