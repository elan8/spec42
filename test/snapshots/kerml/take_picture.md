# META
~~~ini
description=KerML Behavior: TakePicture
type=file
semantic_graph=skip
semantic_graph_skip_reason=KerML behavior, class, and step declarations are opaque parser fallback nodes; their members and relationship endpoints are unavailable as structured semantic inputs
~~~
# SOURCE
~~~kerml
behavior TakePicture {
	private import Camera;
	
	feature camera: Camera[1] subsets involvedObjects;
	
	class Exposure;
	
	behavior Focus { out xrsl: Exposure; }
	behavior Shoot { in xsf: Exposure; }
	
	step step1: Focus[1];	
	step step2: Shoot[1];
	
	succession flow exposure[1] of Exposure from step1.xrsl to step2.xsf;

	succession step1 then camera.focusedState;
	succession step2 then camera.shotState;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "take_picture.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "05c70ae2e850dd796023b18293fbb57cfcf4130d1b7d3ed003d38872dc08df80") (contract-version "canonical-resolution-v1"))
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
