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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "camera.md"
    (diagnostics
    )
  )
)
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
# FORMAT
~~~sysml
class Camera {
	private import ScalarValues::*;
	
	portion focusedState: Camera subsets timeSlices;
	portion shotState: Camera subsets timeSlices;
	
	succession focusedState then shotState;
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8e617e5ca6363e6976abbfbe1a69e618d9dbedfc24b17181f1dd95e832d62b23") (contract-version "canonical-resolution-v1"))
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
