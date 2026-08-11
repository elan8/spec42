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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "63a1d9416e0e67b10556a87afc4a2c2577ecbd981f709571939a8ad9d4a72be4") (contract-version "canonical-resolution-v1"))
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
# NAVIGATION
~~~sexpr
(navigation
)
~~~
