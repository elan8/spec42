# META
~~~ini
description=Connector definitions with references in ends
type=kerml
semantic_graph=skip
semantic_graph_skip_reason=KerML class and connector declarations are opaque parser fallback nodes; connector ends and reference targets are unavailable as structured semantic inputs
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
  (document "connector_references.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2a4a06dba4df13c61bfeb4acb27e870704424838000addf1386ba036de223782") (contract-version "canonical-resolution-v1"))
  (structure
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
