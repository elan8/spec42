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
# TOKENS
~~~zig
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (class_def 'A'
    (feature_def 'self' : 'A')
    (feature_def 'this' : 'A')
    (connector_def : 'HappensDuring'
      (connector_end)
      (connector_end))
    (connector_def : 'InsideOf'
      (connector_end)
      (connector_end))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'InsideOf'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'union'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'InsideOf'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'union'
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "63c3aec65001028ae150ef179956f32fb18390883babeb9963a7ec3ed8a46c89") (contract-version "canonical-resolution-v1"))
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
