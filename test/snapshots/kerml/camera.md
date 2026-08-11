# META
~~~ini
description=KerML Behavior: Camera
type=file
semantic_graph=skip
semantic_graph_skip_reason=KerML class portions and successions are opaque parser fallback nodes; containment and succession endpoints are unavailable as structured semantic inputs
~~~
# SOURCE
~~~kerml
class Camera {
	private import ScalarValues::*;
	
	portion focusedState: Camera subsets timeSlices;
	portion shotState: Camera subsets timeSlices;
	
	succession focusedState then shotState;
}
~~~
# TOKENS
~~~zig
KwClass,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPortion,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
KwPortion,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
KwSuccession,Ident,KwThen,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (class_def 'Camera'
    (import_decl private 'ScalarValues::*')
    (feature_def portion 'focusedState' : 'Camera' :> 'timeSlices')
    (feature_def portion 'shotState' : 'Camera' :> 'timeSlices')
    (succession_def
      (connector_end)
      (connector_end))))
~~~
# FORMAT
~~~sysml
class Camera {
	private import ScalarValues::*;
	
	portion focusedState: Camera subsets timeSlices;
	portion shotState: Camera subsets timeSlices;
	
	succession focusedState then shotState;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
~~~
# SMG
~~~
(semantic-graph
  (containment
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/camera.md"
    (diagnostics
    )
  )
)
~~~
