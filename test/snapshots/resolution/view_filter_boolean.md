# META
~~~ini
description=A view filter that settles to a non-Boolean constant is reported; a package import filter is not
type=file
~~~
# SOURCE
~~~sysml
package Views {
	part def Vehicle;
	view def Overview {
		filter 5;
	}
	view summary : Overview {
		filter 2 < 3;
	}
	package Imported {
		filter 5;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/view_filter_boolean.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "view_filter_non_boolean")
        (source "semantic")
        (range (start 3 9) (end 3 10))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d64f696060a3d8a5413b577ff4929f22d8578fdc4b8528404e83e3ef9ba8c529") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Imported"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Overview"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::summary"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Overview")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::summary"))) (kind featureTyping) (ordinal 0))
      (authored-target "Overview")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Overview")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::summary"))) (target (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Overview"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::summary"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Imported"))) (form package-import) (state literal) (start 9 9) (end 9 10) (value (kind integer) (integer 5)))
    (filter (owner (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Overview"))) (form view) (state literal) (start 3 9) (end 3 10) (value (kind integer) (integer 5)))
    (filter (owner (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::summary"))) (form view) (state evaluated) (start 6 9) (end 6 14) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Overview")))
      (subtype (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::summary")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::summary")))
      (type (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Overview")) (provenance authored))
      (effective-type (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Overview")) (source direct))
      (supertype (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Overview")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/view_filter_boolean.md") (range (start 5 16) (end 5 24)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::summary"))) (kind featureTyping) (ordinal 0) (authored-target "Overview")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_filter_boolean.md") (qualified-name "Views::Overview")))))
    )
  )
)
~~~
