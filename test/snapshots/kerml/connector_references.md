# META
~~~ini
description=Connector definitions with references in ends
type=kerml
~~~
# SOURCE
~~~kerml
class A {
	feature self : A;
	feature this : A;
	connector :HappensDuring
		from [1] self references self
		to [1] this references this;
	connector :InsideOf
		from [0..*] smallerOccurrence references elements
		to [1] largerOccurrence references union;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/connector_references.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 0 0) (end 9 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 0 0) (end 9 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:001058ec9e6d1bb0b353852f260ad84da63f109f327985bcc0021860a6133a43") (contract-version "parser-owned-resolution-v1"))
  (declarations
  )
  (references
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
)
~~~
