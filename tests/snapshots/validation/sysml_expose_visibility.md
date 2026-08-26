# META
~~~ini
description=SysML 8.3.26.2 validateExposeVisibility requires an Expose to have protected visibility
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.26.2 validateExposeVisibility
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.26.2:validateExposeVisibility
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the expose keyword produces an Import with protected visibility.
//
// The violating side has no textual counterpart: SysML concrete syntax admits no visibility
// keyword on an expose, so the rule is observable only as the accepted side pinned here.
package Views {
    part def Component;
    view exposed {
        expose Views::Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_expose_visibility.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_expose_visibility.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:65c9d40f0c6ee6232d4a16b25b7fe0ee8f3f251089e9c9cbcb0884c8500017e7") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_expose_visibility.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_expose_visibility.md") (qualified-name "Views::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_expose_visibility.md") (qualified-name "Views::exposed"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_expose_visibility.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Views::Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_expose_visibility.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Views::Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_expose_visibility.md") (qualified-name "Views::Component")))))
  )
  (relationships
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/sysml_expose_visibility.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_expose_visibility.md") (qualified-name "Views::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_expose_visibility.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_expose_visibility.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_expose_visibility.md") (qualified-name "Views::exposed"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_expose_visibility.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_expose_visibility.md") (qualified-name "Views::exposed")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_expose_visibility.md") (range (start 7 15) (end 7 31)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/sysml_expose_visibility.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Views::Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_expose_visibility.md") (qualified-name "Views::Component")))))
    )
  )
)
~~~
